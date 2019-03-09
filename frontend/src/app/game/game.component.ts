import { Component, OnInit } from '@angular/core';
import {BowlingGame} from "../BowlingGame";
import {GameService} from "../game.service";
import {BowlingThrow} from "../BowlingThrow";
import {BowlingPin} from "../BowlingPin";

@Component({
  selector: 'app-game',
  templateUrl: './game.component.html',
  styleUrls: ['./game.component.less']
})
export class GameComponent implements OnInit {

  game: BowlingGame;
  throws: BowlingThrow[];

  constructor(private gameService: GameService) { }

  ngOnInit() {
    this.createNewGame();
    this.throws = [];
  }

  createNewGame(): void{
    this.gameService.getNewGame()
      .subscribe(newGame => {
        this.game = newGame;
        this.throws = [];
      })
  }

  throwSkill(bowling_throw: BowlingThrow): void{
      this.throws.push(bowling_throw);
      this.gameService.throwSkill(this.game.id, this.throws)
        .subscribe(gameState => {
          this.game = gameState;
          gameState.fallen.map(id => {
            this.game.pins[id].fallen = true
          });
        });
  }

}
