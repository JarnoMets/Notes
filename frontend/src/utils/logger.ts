// Lightweight logger wrapper to centralize logging and allow future enhancements
const isProd = import.meta.env && import.meta.env.PROD

export default {
  error: (...args: unknown[]) => { console.error(...args) },
  warn: (...args: unknown[]) => { console.warn(...args) },
  info: (...args: unknown[]) => { if (!isProd) console.info(...args) },
  debug: (...args: unknown[]) => { if (!isProd) console.debug(...args) },
  // convenience alias
  log: (...args: unknown[]) => { if (!isProd) console.log(...args) },
}
