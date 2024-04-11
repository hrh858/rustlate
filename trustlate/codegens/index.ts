import { trustlate as TranslationsCat } from "./cat";
import { trustlate as TranslationsEn } from "./en";
import { trustlate as TranslationsEs } from "./es";

const translations = {
  "cat": TranslationsCat,
  "es": TranslationsEs,
  "en": TranslationsEn
} as const;

export function trustlate(lang: keyof typeof translations) {
  return translations[lang]
}