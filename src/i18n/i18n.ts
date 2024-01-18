import i18n from "i18next";
import { initReactI18next } from "react-i18next";

// the translations
// (tip move them in a JSON file and import them,
// or even better, manage them separated from your code: https://react.i18next.com/guides/multiple-translation-files)
const resources = {
    en: {
        translation: {
            "User Profiles": "Profiles",
            "Add New Server": "Add New Server",
            "Save": "Save",
            "Connect": "Connect",
            "Fancy Mumble Title": "Fancy Mumble",
            "Unknown User": "Unknown User",
            "Search": "Search",
            "Search Channel": "Search Channel",
            "Are you sure you want to delete all messages?": "Are you sure you want to delete all messages?",
            "Yes": "Yes",
            "No": "No",
            "Image too large": "[[ Image too large ( {{size}} out of {{maximum}}) ]]",
            "Like": "Like",
            "Timestamp": "Timestamp",
            "Message": "Message",
            "Search Tenor": "Search Tenor for GIFs",
            "Open In Browser": "Open In Browser",
            "Muted": "Muted",
            "Deafened": "Deafened",
            "Joined": "Joined",
            "User ID": "User ID",
            "write user a message": "write {{user}}...",
            "write something": "Write something :)",
            "Delete all messages": "Delete all messages",
            "Send Message to Channel": "Send Message to {{channel}}",
            "User Joined the Server": "{{user}} joined the server",
            "en": "English",
            "de": "Deutsch",
            "fr": "Français",
            "dev": "Development",
        }
    },
    de: {
        translation: {
            "User Profiles": "Profile",
            "Add New Server": "Neuen Server hinzufügen",
            "Save": "Speichern",
            "Connect": "Verbinden",
            "Fancy Mumble Title": "Fancy Mumble",
            "Unknown User": "Unbekannter Benutzer",
            "Search": "Suche",
            "Search Channel": "Suche Kanal",
            "Are you sure you want to delete all messages?": "Bist du dir sicher, dass Sie alle Nachrichten löschen möchtest?",
            "Yes": "Ja",
            "No": "Nein",
            "Image too large": "[[ Bild zu groß ( {{size}} von maximal {{maximum}}) ]]",
            "Like": "Gefällt mir",
            "Timestamp": "Zeitstempel",
            "Message": "Nachricht",
            "Search Tenor": "Durchsuche Tenor nach GIFs",
            "Open In Browser": "Im Browser öffnen",
            "Muted": "Stumm",
            "Deafened": "Taub",
            "Joined": "Beigetreten",
            "User ID": "Benutzer-ID",
            "write user a message": "Nachricht an {{user}}...",
            "write something": "Schreibe etwas :)",
            "Delete all messages": "Lösche alle Nachrichten",
            "Send Message to Channel": "Nachricht an {{channel}} senden",
            "User Joined the Server": "{{user}} ist dem Server beigetreten",
            "en": "English",
            "de": "Deutsch",
            "fr": "Français",
            "dev": "Development",
        }
    },
    fr: {
        translation: {
            "User Profiles": "Profils",
            "Add New Server": "Ajouter un nouveau serveur",
            "Save": "Enregistrer",
            "Connect": "Connecter",
            "Fancy Mumble Title": "Fancy Mumble",
            "Unknown User": "Utilisateur inconnu",
            "Search": "Chercher",
            "Search Channel": "Chercher un canal",
            "Are you sure you want to delete all messages?": "Êtes-vous sûr de vouloir supprimer tous les messages?",
            "Yes": "Oui",
            "No": "Non",
            "Image too large": "[[ Image trop grande ( {{size}} sur {{maximum}}) ]]",
            "Like": "Aimer",
            "Timestamp": "Horodatage",
            "Message": "Message",
            "Search Tenor": "Chercher Tenor pour des GIFs",
            "Open In Browser": "Ouvrir dans le navigateur",
            "Muted": "Muet",
            "Deafened": "Sourd",
            "Joined": "Rejoint",
            "User ID": "ID utilisateur",
            "write user a message": "écrire à {{user}}...",
            "write something": "Écrivez quelque chose :)",
            "Delete all messages": "Supprimer tous les messages",
            "Send Message to Channel": "Envoyer un message à {{channel}}",
            "User Joined the Server": "{{user}} a rejoint le serveur",
            "en": "English",
            "de": "Deutsch",
            "fr": "Français",
            "dev": "Development",
        }
    },
};

i18n
    .use(initReactI18next) // passes i18n down to react-i18next
    .init({
        resources,
        lng: "en", // language to use, more information here: https://www.i18next.com/overview/configuration-options#languages-namespaces-resources
        // you can use the i18n.changeLanguage function to change the language manually: https://www.i18next.com/overview/api#changelanguage
        // if you're using a language detector, do not define the lng option
        fallbackLng: ["en", "de", "fr", "dev"],
        interpolation: {
            escapeValue: false // react already safes from xss
        }
    });

export default i18n;