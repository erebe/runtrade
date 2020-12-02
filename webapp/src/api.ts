import Axios, { AxiosResponse } from "axios";

export function findEventByName(eventName: string): Promise<AxiosResponse> {
    return Axios.get('http://localhost:8081/api/v1/events/' + encodeURI(eventName));
}

export function getInscriptionForEvent(eventId: number): Promise<AxiosResponse> {
    return Axios.get('http://localhost:8081/api/v1/inscriptions/event_id/' + eventId);
}
