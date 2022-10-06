import { Injectable } from '@angular/core';
import { User } from './users/user';
import { Observable, of, throwError } from 'rxjs';
import { catchError, retry } from 'rxjs/operators';
import { HttpClient } from '@angular/common/http';
import { HttpErrorHandler, HandleError } from './http-error-handler.service';

export interface Config {
  usersUrl: string;
}

@Injectable({
  providedIn: 'root'
})
export class UsersService {
  usersUrl = 'http://127.0.0.1:8080/users';  // URL to web api
  configUrl = 'assets/config.json';
  private handleError: HandleError;

  constructor(private http: HttpClient,
    httpErrorHandler: HttpErrorHandler) {
    this.handleError = httpErrorHandler.createHandleError('UsersService');
   }

  getConfig() {
    return this.http.get<Config>(this.configUrl);
  }

  getUsers(): Observable<User[]> {
    return this.http.get<User[]>(this.usersUrl)
    .pipe(
      catchError(this.handleError('getUsers', []))
    );
  }
}
