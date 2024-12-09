// www/index.js
import init, { Game, Renderer } from '../pkg/treasure_hunt_wasm.js';

let game = null;
let renderer = null;
let animationId = null;
//跟踪事件是否已初始化
let eventsInitialized = false;

function initializeEvents() {
    if (!eventsInitialized) {
        document.addEventListener('keydown', handleKeyDown);
        document.addEventListener('keyup', handleKeyUp);
        eventsInitialized = true;
    }
}

async function startGame() {
    if (!game) {
        const canvas = document.getElementById('game-canvas');
        const width = 800;
        const height = 600;

        game = new Game(width, height);
        renderer = new Renderer(canvas, width, height);
         // 只在首次创建游戏时初始化事件
        initializeEvents();
        
        // 添加键盘事件监听
        document.addEventListener('keydown', handleKeyDown);
        document.addEventListener('keyup', handleKeyUp);
    } else {
        game.reset();
    }
    
    game.start();
    console.log("Game started");
    gameLoop();
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
        console.log("GameLoop timestamp:", timestamp); 
        game.update(timestamp);  // 直接传递 timestamp
        renderer.render(game);
        animationId = requestAnimationFrame(gameLoop);
    }
}

// www/index.js
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

// 确保在页面加载完成后初始化
window.addEventListener('load', initialize);