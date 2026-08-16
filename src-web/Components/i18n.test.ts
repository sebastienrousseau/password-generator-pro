import { describe, expect, it } from 'vitest'

import enLocaleJSON from '../Locales/en-GB.json'
import frLocaleJSON from '../Locales/fr-FR.json'
import Translate, { FALLBACK_LOCALE, resolveLocale } from './i18n'

// Indexed by runtime keys, so widen away the inferred literal type.
const enLocale: Record<string, string> = enLocaleJSON
const frLocale: Record<string, string> = frLocaleJSON

/**
 * Every key App.svelte asks for at start-up.
 *
 * If any of these is missing from a shipped locale the user sees an
 * untranslated string, which is survivable — but before the fallback
 * below existed, a missing *locale* threw and the window stayed blank.
 */
const KEYS_USED_BY_THE_APP = [
  'Button',
  'Button2',
  'Label',
  'Label2',
  'Label3',
  'Label4',
  'Label5',
  'Placeholder',
  'Placeholder2',
  'Placeholder3',
  'Subtitle',
  'Title',
]

describe('resolveLocale', () => {
  it('returns a shipped tag unchanged', () => {
    expect(resolveLocale('fr-FR')).toBe('fr-FR')
    expect(resolveLocale('en-GB')).toBe('en-GB')
    expect(resolveLocale('zh-CN')).toBe('zh-CN')
  })

  it('falls back to the same language when the region is unshipped', () => {
    // fr-XX is not in the table, but French is.
    expect(resolveLocale('fr-XX').startsWith('fr-')).toBe(true)
    expect(resolveLocale('de-XX').startsWith('de-')).toBe(true)
  })

  it('falls back to en-GB for a language we do not ship at all', () => {
    // These are real system locales with no table in this repo, and
    // each one used to blank the application.
    for (const locale of ['nb-NO', 'he-IL', 'vi-VN', 'fa-IR', 'ms-MY']) {
      expect(resolveLocale(locale)).toBe(FALLBACK_LOCALE)
    }
  })

  it('falls back for malformed input rather than throwing', () => {
    for (const locale of ['', '-', 'not a locale', 'en_GB', '???']) {
      expect(() => resolveLocale(locale)).not.toThrow()
      expect(typeof resolveLocale(locale)).toBe('string')
    }
  })
})

describe('Translate', () => {
  it('returns the translation for a shipped locale', () => {
    expect(Translate('Button', 'fr-FR')).toBe(frLocale['Button'])
    expect(Translate('Button', 'en-GB')).toBe(enLocale['Button'])
  })

  it('returns French for an unshipped French region', () => {
    expect(Translate('Button', 'fr-XX')).toBe(frLocale['Button'])
  })

  it('never throws for any locale, shipped or not', () => {
    const locales = ['en-GB', 'fr-FR', 'nb-NO', 'he-IL', '', 'zz-ZZ', 'xx']
    for (const locale of locales) {
      for (const key of KEYS_USED_BY_THE_APP) {
        expect(() => Translate(key, locale)).not.toThrow()
        expect(Translate(key, locale)).toBeTypeOf('string')
      }
    }
  })

  it('returns a non-empty string for every key the app renders', () => {
    // This is the regression: the app called Translate twelve times in
    // its script block, and one undefined lookup killed all twelve.
    for (const key of KEYS_USED_BY_THE_APP) {
      expect(Translate(key, 'nb-NO')).not.toBe('')
      expect(Translate(key, 'nb-NO')).toBe(enLocale[key])
    }
  })

  it('falls back to the key itself for a key nothing defines', () => {
    expect(Translate('NoSuchKey', 'en-GB')).toBe('NoSuchKey')
    expect(Translate('NoSuchKey', 'nb-NO')).toBe('NoSuchKey')
  })

  it('serves every app key from the reference locale', () => {
    for (const key of KEYS_USED_BY_THE_APP) {
      expect(enLocale[key], `en-GB is missing ${key}`).toBeDefined()
    }
  })
})
