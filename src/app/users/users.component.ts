import { Component, OnInit } from '@angular/core';
import { Config, UsersService } from '../users.service';
import { User } from './user';

@Component({
  selector: 'app-users',
  templateUrl: './users.component.html',
  styleUrls: ['./users.component.scss']
})
export class UsersComponent implements OnInit {
  config: Config | undefined = undefined;
  users: User[] = [];

  constructor(private usersService: UsersService) { }

  showConfig() {
    this.usersService.getConfig()
      .subscribe((data: Config) => this.config = {
          usersUrl: data.usersUrl
      });
  }

  getUsers() {
    this.usersService.getUsers()
    .subscribe((data: User[]) => {
      this.users = data
      console.log(this.users)
    });
  }

  ngOnInit(): void {
    this.getUsers()
  }

}
