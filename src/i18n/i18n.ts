import i18n from "i18next";
import { initReactI18next } from "react-i18next";
import Backend from 'i18next-http-backend';

i18n
    .use(initReactI18next) // passes i18n down to react-i18next
    .use(Backend)
    .init({
        lng: "en", // language to use, more information here: https://www.i18next.com/overview/configuration-options#languages-namespaces-resources
        // you can use the i18n.changeLanguage function to change the language manually: https://www.i18next.com/overview/api#changelanguage
        // if you're using a language detector, do not define the lng option
        fallbackLng: ["de", "en", "fr", "es", "zh", "dev"],
        ns: ['common', 'appearance', 'audio', 'language', 'notifications', 'privacy', 'time', 'user_interaction'],
        defaultNS: 'common',
        interpolation: {
            escapeValue: false // react already safes from xss
        },
        backend: {
            loadPath: '/locales/{{lng}}/{{ns}}.json'
        }
    });

export default i18n;