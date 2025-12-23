import { Mark } from '@tiptap/core'
import { mergeAttributes } from '@tiptap/core'

export interface BBCodeLinkOptions {
  openOnClick: boolean
  HTMLAttributes: Record<string, any>
}

declare module '@tiptap/core' {
  interface Commands<ReturnType> {
    bbcodeLink: {
      /**
       * Set a link mark with bbcode reference
       */
      setBBCodeLink: (options: {
        href: string
        type: 'note' | 'board'
        referencedId: string
        referencedName: string
      }) => ReturnType
      /**
       * Toggle a link mark
       */
      toggleBBCodeLink: (options: {
        href: string
        type: 'note' | 'board'
        referencedId: string
        referencedName: string
      }) => ReturnType
      /**
       * Unset a link mark
       */
      unsetBBCodeLink: () => ReturnType
    }
  }
}

export const BBCodeLink = Mark.create<BBCodeLinkOptions>({
  name: 'bbcodeLink',

  addOptions() {
    return {
      openOnClick: true,
      HTMLAttributes: {},
    }
  },

  addAttributes() {
    return {
      href: {
        default: null,
        parseHTML: element => element.getAttribute('href'),
        renderHTML: attributes => {
          return {
            href: attributes.href,
          }
        },
      },
      type: {
        default: 'note',
        parseHTML: element => element.getAttribute('data-type') || 'note',
        renderHTML: attributes => {
          return {
            'data-type': attributes.type,
          }
        },
      },
      referencedId: {
        default: null,
        parseHTML: element => element.getAttribute('data-referenced-id'),
        renderHTML: attributes => {
          return {
            'data-referenced-id': attributes.referencedId,
          }
        },
      },
      referencedName: {
        default: null,
        parseHTML: element => element.getAttribute('data-referenced-name'),
        renderHTML: attributes => {
          return {
            'data-referenced-name': attributes.referencedName,
          }
        },
      },
    }
  },

  parseHTML() {
    return [
      {
        tag: 'a[data-bbcode-type]',
      },
    ]
  },

  renderHTML({ HTMLAttributes }) {
    return ['a', mergeAttributes(this.options.HTMLAttributes, HTMLAttributes, {
      class: `bbcode-link bbcode-${HTMLAttributes['data-type'] || 'note'}-link`,
    }), 0]
  },

  addCommands() {
    return {
      setBBCodeLink: (options) => ({ commands }) => {
        return commands.setMark(this.name, {
          href: options.href,
          type: options.type,
          referencedId: options.referencedId,
          referencedName: options.referencedName,
        })
      },

      toggleBBCodeLink: (options) => ({ commands }) => {
        return commands.toggleMark(this.name, {
          href: options.href,
          type: options.type,
          referencedId: options.referencedId,
          referencedName: options.referencedName,
        })
      },

      unsetBBCodeLink: () => ({ commands }) => {
        return commands.unsetMark(this.name)
      },
    }
  },

  addKeyboardShortcuts() {
    return {
      'Mod-k': () => {
        return false
      },
    }
  },
})
