import Axios, { AxiosResponse } from "axios";

export function findEventByName(eventName: string): Promise<AxiosResponse> {
    return Axios.get('http://localhost:8081/api/v1/events/search/' + encodeURI(eventName));
}

export function addEvent(event: any): Promise<AxiosResponse> {
    return Axios.put('http://localhost:8081/api/v1/event', event);
}

export function getEventTypes(): Promise<AxiosResponse> {
    return Axios.get('http://localhost:8081/api/v1/event/types');
}

export function getInscriptionForEvent(eventId: number): Promise<AxiosResponse> {
    return Axios.get('http://localhost:8081/api/v1/inscriptions/event_id/' + eventId);
}

export function eventTypeToSvgIconPath(eventType: string) {
    switch (eventType.toLocaleLowerCase()) {
        case 'run':
            return '/icons/run.svg'
        case 'trail':
            return '/icons/trail.svg'
        case 'bike':
            return '/icons/bike.svg'
        default:
            return '/icons/other.svg'
    }
}
