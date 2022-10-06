import { Component, OnInit } from '@angular/core';
import { FormBuilder } from '@angular/forms';

@Component({
  selector: 'app-sign-up',
  templateUrl: './sign-up.component.html',
  styleUrls: ['./sign-up.component.scss']
})
export class SignUpComponent implements OnInit {

  signUpForm = this.formBuilder.group({
    name: '',
    password: ''
  });


  constructor(private formBuilder: FormBuilder) { }

  onSubmit(): void {
    // Process sign up data here
    console.warn('Your order has been submitted', this.signUpForm.value);
    this.signUpForm.reset();
  }

  ngOnInit(): void {
  }

}
