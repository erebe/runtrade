/* eslint-disable @typescript-eslint/camelcase */
import Axios, {AxiosResponse} from "axios";

export interface User {
    id?: string;
    name: string;
    contact: string;
    email: string;
    last_logged: number;
}

export function newUser(): User {
    return {contact: "", email: "", last_logged: Math.floor((new Date()).getTime() / 1000), name: ""};
}

export interface Event {
    id?: number;
    name: string;
    event_type: string;
    localisation: string;
    event_date: number | string;
    event_link: string;
    created_at: number;
    user_id: string;
}

export function newEvent(user: User): Event {
    return {
        created_at: Math.floor((new Date()).getTime() / 1000),
        event_date: "",
        event_link: "",
        event_type: "",
        localisation: "",
        name: "",
        user_id: user.id!
    };
}

export interface Inscription {
    id?: number;
    user_id: string;
    event_id: number;
    category: string;
    price: number;
    currency: string;
    intent: string;
    created_at: number;
    note: string;
    gender: string;
}

export function newInscription(user: User, event: Event): Inscription {
    return {
        created_at: Math.floor((new Date()).getTime() / 1000),
        category: "",
        currency: "€",
        event_id: event.id!,
        gender: "Man",
        intent: "Sell",
        note: "",
        price: 0,
        user_id: user.id!
    };
}

const HOSTNAME = process.env.API_HOSTNAME;
export function deleteInscription(inscriptionID: number): Promise<AxiosResponse<boolean>> {
    return Axios.delete(HOSTNAME + '/api/v1/inscription/' + inscriptionID);
}

export function updateUserContact(user: User, contact: string): Promise<AxiosResponse<User>> {
    return Axios.put(HOSTNAME + '/api/v1/user/contact',
        JSON.stringify(contact),
        {headers: {'Content-Type': 'application/json'}}
    );
}

export function findEventByName(eventName: string): Promise<AxiosResponse<Array<Event>>> {
    return Axios.get(HOSTNAME + '/api/v1/events/search/' + encodeURI(eventName));
}

export function userLogged(user: User): Promise<AxiosResponse<User>> {
    return Axios.put(HOSTNAME + '/api/v1/user/logged', user);
}

export function addEvent(event: Event): Promise<AxiosResponse<Event>> {
    return Axios.put(HOSTNAME + '/api/v1/event', event);
}

export function addTrade(trade: Inscription): Promise<AxiosResponse<Inscription>> {
    return Axios.put(HOSTNAME + '/api/v1/inscription', trade);
}

export function getEvent(eventId: number): Promise<AxiosResponse<Event>> {
    return Axios.get(HOSTNAME + '/api/v1/event/' + eventId);
}

export function getEventTypes(): Promise<AxiosResponse> {
    return Axios.get(HOSTNAME + '/api/v1/event/types');
}

export function getInscriptionForEvent(eventId: number): Promise<AxiosResponse<Array<[User, Inscription, Event]>>> {
    return Axios.get(HOSTNAME + '/api/v1/inscriptions/event_id/' + eventId);
}

export function eventTypeToSvgIconPath(eventType: string) {
    switch (eventType.toLocaleLowerCase()) {
        case 'run':
            return '/icons/run.svg'
        case 'trail':
            return '/icons/trail.svg'
        case 'bike':
            return '/icons/bike.svg'
        case 'triathlon':
            return '/icons/triathlon.svg'
        default:
            return '/icons/other.svg'
    }
}
