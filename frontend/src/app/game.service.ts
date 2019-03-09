import { Injectable } from '@angular/core';
import {Observable, of} from "rxjs";
import { HttpClient, HttpHeaders} from "@angular/common/http";
import { catchError, map, tap} from "rxjs/operators";
import { MessageService } from './message.service';
import {BowlingGame} from "./BowlingGame";
import {BowlingThrow} from "./BowlingThrow";

@Injectable({
  providedIn: 'root'
})
export class GameService {

  constructor(
    private http: HttpClient,
    private messageService: MessageService
  ) { }

  private newGameApi = '/api/new_game';
  private playApi = ('/api/play')

  defaultGame: BowlingGame = {
    id: 0,
    pins: [],
    throws: []
  };


  getNewGame(): Observable<BowlingGame> {
    return this.http.get<BowlingGame>(this.newGameApi)
      .pipe(
        tap( _ => this.log("new game fetched")),
        catchError(this.handleError('getNewGame', this.defaultGame))
      )
  }

  throwSkill(game_id: number, throws: BowlingThrow[]): Observable<BowlingGame> {
    return this.http.post<BowlingGame>(this.playApi, {'game': game_id, 'throws': throws.map(a => a.id)})
      .pipe(
        tap( _ => this.log('skill thrown and new game state fethed')),
        catchError(this.handleError('throwSkill', this.defaultGame))
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
    this.messageService.add(`GameService: ${message}`);
  }
}
