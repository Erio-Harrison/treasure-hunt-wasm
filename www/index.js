import init, { Game, Renderer } from '../pkg/treasure_hunt_wasm.js';

let game = null;
let renderer = null;
let animationId = null;
let eventsInitialized = false;

function initializeEvents() {
    if (!eventsInitialized) {
        document.addEventListener('keydown', handleKeyDown);
        document.addEventListener('keyup', handleKeyUp);
        eventsInitialized = true;
    }
}


async function createGame(width, height) {
    try {
        return await new Game(width, height);
    } catch (error) {
        console.error("Failed to create game:", error);
        throw error;
    }
}

async function startGame() {
    try {
        if (!game) {
            const canvas = document.getElementById('game-canvas');
            const width = 800;
            const height = 600;

            // 创建新的游戏实例
            game = await createGame(width, height);
            
            if (!game) {
                throw new Error("Failed to create game instance");
            }

            renderer = new Renderer(canvas, width, height);
            initializeEvents();
        } else {
            game.reset();
        }

        if (game && typeof game.start === 'function') {
            game.start();
            console.log("Game started");
            gameLoop();
        } else {
            throw new Error("Game instance not properly initialized");
        }
    } catch (error) {
        console.error("Error starting game:", error);
        console.error("Game object:", game);
    }
}

function stopGame() {
    if (game) {
        game.stop();
        console.log("Game stopped");
        if (animationId) {
            cancelAnimationFrame(animationId);
            animationId = null;
        }
    }
}

function handleKeyDown(event) {
    if (game && game.is_running()) {
        game.handle_key_down(event.key);
    }
}

function handleKeyUp(event) {
    if (game && game.is_running()) {
        game.handle_key_up(event.key);
    }
}

function gameLoop(timestamp) {
    if (game && game.is_running()) {
        if (timestamp === undefined) {
            timestamp = 0;
        }
        
        game.update(timestamp);
        renderer.render(game);
        animationId = requestAnimationFrame(gameLoop);
    }
}

async function initialize() {
    try {
        await init();
        console.log("WASM initialized");

        const startButton = document.getElementById('start-button');
        const stopButton = document.getElementById('stop-button');
        const resetButton = document.getElementById('reset-button');

        if (startButton) {
            startButton.onclick = startGame;
        }
        if (stopButton) {
            stopButton.onclick = stopGame;
        }
        if (resetButton) {
            resetButton.onclick = startGame; // 重用 startGame 作为重置功能
        }

        console.log("Buttons initialized");
    } catch (error) {
        console.error("Initialization error:", error);
    }
}

window.addEventListener('load', initialize);