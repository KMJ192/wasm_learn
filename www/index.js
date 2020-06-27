//import * as wasm from "wasm-snake-game";
import { GameManager } from './src/game-manager'

//wasm.greet();
const gameManager = new GameManager();
gameManager.run();