import { BowlingPin } from "./BowlingPin";
import { BowlingThrow } from "./BowlingThrow";

export class BowlingGame{
  id: number;
  pins: BowlingPin[];
  fallen: number[];
  throws: BowlingThrow[];
}
