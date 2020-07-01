import { Game, Vector } from 'wasm-snake-game'
import CONFIG from './config'
import { View } from './view'
import { Controller } from './controller'

export class GameManager{
    constructor(){
        this.restart()
        this.view = new View(
            this.game.width,
            this.game.height,
            this.render.bind(this)
        )
        this.controller = new Controller(
            this.onStop.bind(this)
        )
    }

    restart(){
        this.game = new Game(
            CONFIG.WIDTH,
            CONFIG.HEIGHT,
            CONFIG.SPEED,
            CONFIG.SNAKE_LENGTH,
            new Vector(
                CONFIG.SNAKE_DIRECTION_X,
                CONFIG.SNAKE_DIRECTION_Y
            )
        )
        console.log(this.game)
    }

    onStop(){
        const now = Data.new()
        if (this.stopTime) {
            this.stopTime = undefined
            this.lastUpdate = this.time + now - this.lastUpdate
        }else{
            this.stopTime = now
        }
    }

    render(){
        this.view.render(
            this.game.food,
            this.game.get_snake(),
            this.game.score,
            //TODO actual best score
            0
        )
    }

    tick(){
        if(!this.stopTime){
            const lastUpdate = Date.now();
            if(this.lastUpdate){
                this.game.process(lastUpdate - this.lastUpdate, this.controller.movement)
            }
            this.lastUpdate = lastUpdate
            this.render()
        }
    }

    run(){
        setInterval(this.tick.bind(this), 1000 / CONFIG.FPS)
    }
}