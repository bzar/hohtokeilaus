import { Component, OnInit } from '@angular/core';
import {BowlingGame} from "../BowlingGame";
import {GameService} from "../game.service";

@Component({
  selector: 'app-game',
  templateUrl: './game.component.html',
  styleUrls: ['./game.component.less']
})
export class GameComponent implements OnInit {

  game: BowlingGame;
  constructor(private gameService: GameService) { }

  ngOnInit() {
    this.createNewGame();
  }

  createNewGame(): void{
    this.gameService.getNewGame()
      .subscribe(newGame => this.game = newGame)
  }
}
