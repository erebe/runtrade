/* eslint-disable @typescript-eslint/camelcase */
import Axios from "axios";
export function newUser() {
    return { contact: "", email: "", last_logged: Math.floor((new Date()).getTime() / 1000), name: "" };
}
export function newEvent(user) {
    return {
        created_at: Math.floor((new Date()).getTime() / 1000),
        event_date: "",
        event_link: "",
        event_type: "",
        localisation: "",
        name: "",
        user_id: user.id
    };
}
export function newInscription(user, event) {
    return {
        created_at: Math.floor((new Date()).getTime() / 1000),
        category: "",
        currency: "€",
        event_id: event.id,
        gender: "Man",
        intent: "Sell",
        note: "",
        price: 0,
        user_id: user.id
    };
}
const HOSTNAME = '';
// const HOSTNAME = 'http://localhost:8081';
export function deleteInscription(inscriptionID) {
    return Axios.delete(HOSTNAME + '/api/v1/inscription/' + inscriptionID);
}
export function updateUserContact(user, contact) {
    return Axios.put(HOSTNAME + '/api/v1/user/contact', JSON.stringify(contact), { headers: { 'Content-Type': 'application/json' } });
}
export function findEventByName(eventName) {
    return Axios.get(HOSTNAME + '/api/v1/events/search/' + encodeURI(eventName));
}
export function userLogged(user) {
    return Axios.put(HOSTNAME + '/api/v1/user/logged', user);
}
export function addEvent(event) {
    return Axios.put(HOSTNAME + '/api/v1/event', event);
}
export function addTrade(trade) {
    return Axios.put(HOSTNAME + '/api/v1/inscription', trade);
}
export function getEvent(eventId) {
    return Axios.get(HOSTNAME + '/api/v1/event/' + eventId);
}
export function getEventTypes() {
    return Axios.get(HOSTNAME + '/api/v1/event/types');
}
export function getInscriptionForEvent(eventId) {
    return Axios.get(HOSTNAME + '/api/v1/inscriptions/event_id/' + eventId);
}
export function eventTypeToSvgIconPath(eventType) {
    switch (eventType.toLocaleLowerCase()) {
        case 'run':
            return '/icons/run.svg';
        case 'trail':
            return '/icons/trail.svg';
        case 'bike':
            return '/icons/bike.svg';
        default:
            return '/icons/other.svg';
    }
}
//# sourceMappingURL=api.js.map