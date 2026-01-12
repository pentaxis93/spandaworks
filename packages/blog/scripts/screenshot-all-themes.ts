import puppeteer from 'puppeteer';
import { spawn } from 'child_process';
import net from 'net';

const PREFERRED_PORT = Number(process.env.SCREENSHOT_PORT ?? process.env.PORT ?? 4321);
const MAX_PORT_ATTEMPTS = 20;

const THEMES = [
  { name: 'workbench', file: 'workbench-screenshot.png' }
];

async function checkServer(port: number): Promise<boolean> {
  try {
    const response = await fetch(`http://localhost:${port}`);
    return response.ok;
  } catch (e) {
    return false;
  }
}

function isPortAvailable(port: number): Promise<boolean> {
  return new Promise(resolve => {
    const tester = net.createServer();
    tester.unref();
    tester.once('error', () => resolve(false));
    tester.once('listening', () => tester.close(() => resolve(true)));
    tester.listen(port, '127.0.0.1');
  });
}

async function resolvePort(preferred: number): Promise<number> {
  if (await isPortAvailable(preferred)) {
    return preferred;
  }

  for (let offset = 1; offset <= MAX_PORT_ATTEMPTS; offset++) {
    const candidate = preferred + offset;
    if (await isPortAvailable(candidate)) {
      return candidate;
    }
  }

  throw new Error(`No available port found near ${preferred}.`);
}

async function startServer(port: number) {
  console.log(`Starting dev server on port ${port}...`);
  const server = spawn('bun', ['run', 'dev', '--', '--port', String(port)], {
    stdio: 'inherit',
    detached: true
  });

  let attempts = 0;
  while (attempts < 30) {
    if (await checkServer(port)) {
      console.log('Server is ready!');
      return server;
    }
    await new Promise(r => setTimeout(r, 1000));
    attempts++;
  }

  throw new Error('Server failed to start');
}

(async () => {
  let serverProcess: ReturnType<typeof spawn> | null = null;
  const port = await resolvePort(PREFERRED_PORT);
  const url = `http://localhost:${port}`;

  try {
    serverProcess = await startServer(port);

    const browser = await puppeteer.launch({
      headless: true,
      args: ['--no-sandbox', '--disable-setuid-sandbox']
    });
    const page = await browser.newPage();

    await page.setViewport({ width: 1280, height: 800 });

    console.log(`Navigating to ${url}...`);
    await page.goto(url, { waitUntil: 'networkidle0' });

    // Capture screenshots of all three themes
    for (const theme of THEMES) {
      console.log(`Switching to theme: ${theme.name}`);
      
      // Set the theme via localStorage and reload
      await page.evaluate((themeName) => {
        localStorage.setItem('theme', themeName);
        document.documentElement.setAttribute('data-theme', themeName);
      }, theme.name);

      // Wait a moment for theme to apply
      await new Promise(r => setTimeout(r, 500));

      await page.screenshot({ path: theme.file, fullPage: true });
      console.log(`Screenshot saved to ${theme.file}`);
    }

    await browser.close();
    console.log('\nAll theme screenshots captured successfully!');
  } catch (error) {
    console.error('Error taking screenshots:', error);
  } finally {
    if (serverProcess) {
      console.log('Stopping temporary server...');
      try {
        process.kill(-serverProcess.pid!);
      } catch (err) {
        // Ignore if process group kill is unsupported.
      }
      serverProcess.kill();
    }
  }
})();
