export type FloatingMenuItem = {
  label?: string
  action?: () => void
  divider?: boolean
  danger?: boolean
  icon?: string
}

function createMenuElement(x: number, y: number) {
  const menu = document.createElement('div')
  menu.className = 'context-menu'
  menu.style.left = `${x}px`
  menu.style.top = `${y}px`
  return menu
}

export function closeAllFloatingMenus() {
  if (typeof document === 'undefined') return
  const existing = Array.from(document.querySelectorAll('.context-menu'))
  for (const el of existing) {
    el.remove()
  }
}

export function openFloatingMenu(items: FloatingMenuItem[], x: number, y: number) {
  if (typeof document === 'undefined') return { close: () => {} }

  // Close existing menus so only one is visible.
  closeAllFloatingMenus()

  const menu = createMenuElement(x, y)

  for (const item of items) {
    if (item.divider) {
      const divider = document.createElement('div')
      divider.className = 'context-menu-divider'
      menu.appendChild(divider)
      continue
    }

    const el = document.createElement('div')
    el.className = 'context-menu-item'
    el.textContent = item.label ?? ''
    if (item.danger) el.classList.add('danger')
    el.addEventListener('click', () => {
      try { item.action && item.action() } catch (e) { console.error('floating menu action failed', e) }
      if (menu.parentNode) menu.parentNode.removeChild(menu)
      cleanup()
    })
    el.addEventListener('mouseenter', () => el.style.background = 'var(--bg-hover, #f5f5f5)')
    el.addEventListener('mouseleave', () => el.style.background = 'transparent')
    menu.appendChild(el)
  }

  document.body.appendChild(menu)

  const onDocClick = (ev: MouseEvent) => {
    if (!menu.contains(ev.target as Node)) {
      if (menu.parentNode) menu.parentNode.removeChild(menu)
      cleanup()
    }
  }

  const onKey = (ev: KeyboardEvent) => {
    if (ev.key === 'Escape') {
      if (menu.parentNode) menu.parentNode.removeChild(menu)
      cleanup()
    }
  }

  function cleanup() {
    document.removeEventListener('click', onDocClick)
    document.removeEventListener('keydown', onKey)
  }

  // Delay attaching the click handler so the originating click doesn't immediately close it
  setTimeout(() => document.addEventListener('click', onDocClick), 0)
  document.addEventListener('keydown', onKey)

  return {
    close() {
      if (menu.parentNode) menu.parentNode.removeChild(menu)
      cleanup()
    }
  }
}

export default { openFloatingMenu, closeAllFloatingMenus }
