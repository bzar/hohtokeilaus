import { Injectable } from '@angular/core';
import { Profile } from "./profile";
import { Observable, of } from "rxjs";
import { HttpClient, HttpHeaders} from "@angular/common/http";
import { catchError, map, tap} from "rxjs/operators";
import { MessageService } from './message.service';

@Injectable({
  providedIn: 'root'
})
export class ProfileService {

  profile: Profile = {
    id: 1,
    name: "Seppo"
  }

  private profileUrl: string = 'api/me';

  constructor(
    private http: HttpClient,
    private messageService: MessageService) { }

  getProfile(): Observable<Profile> {
    return this.http.get<Profile>(this.profileUrl)
      .pipe(
        tap(_ => this.log('fetched profile')),
        catchError(this.handleError('getProfile', this.profile))
      )
  }

  /**
   * Handle Http operation that failed.
   * Let the app continue.
   * @param operation - name of the operation that failed
   * @param result - optional value to return as the observable result
   */
  private handleError<T> (operation = 'operation', result?: T) {
    return (error: any): Observable<T> => {

      // TODO: send the error to remote logging infrastructure
      console.error(error); // log to console instead

      // TODO: better job of transforming error for user consumption
      this.log(`${operation} failed: ${error.message}`);

      // Let the app keep running by returning an empty result.
      return of(result as T);
    };
  }

  private log(message: string) {
    this.messageService.add(`HeroService: ${message}`);
  }
}
