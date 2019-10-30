import { memory } from "wasm-game-of-life/wasm_game_of_life_bg";
import { Universe, Cell } from "wasm-game-of-life";

const CELL_SIZE = 5; // Units in px
const CELL_BORDER_SIZE = 1; // Units in px
const GRID_COLOR = '#CCCCCC';
const DEAD_COLOR = '#FFFFFF';
const ALIVE_COLOR = '#000000';

const universe = Universe.new();
const width = universe.width();
const height = universe.height();

const canvas = document.querySelector('#game-of-life-canvas');
canvas.width = (CELL_SIZE + CELL_BORDER_SIZE) * height + CELL_BORDER_SIZE;
canvas.height = (CELL_SIZE + CELL_BORDER_SIZE) * width + CELL_BORDER_SIZE;

const context = canvas.getContext('2d');

const renderLoop = () => {
    universe.tick();

    drawGrid();
    drawCells();
    requestAnimationFrame(renderLoop);
}

requestAnimationFrame(renderLoop);


const drawGrid = () => {
    context.beginPath();
    context.strokeStyle = GRID_COLOR;

    // Vertical Lines
    for (let i = 0; i <= width; i++) {
        context.moveTo(i * (CELL_SIZE + CELL_BORDER_SIZE) + CELL_BORDER_SIZE, 0);
        context.lineTo(i * (CELL_SIZE + CELL_BORDER_SIZE) + CELL_BORDER_SIZE, (CELL_SIZE + CELL_BORDER_SIZE) * height + CELL_BORDER_SIZE);
    }

    // Horizontal Lines
    for (let j = 0; j <= height; j++) {
        context.moveTo(0,                                                         j * (CELL_SIZE + CELL_BORDER_SIZE) + CELL_BORDER_SIZE);
        context.lineTo((CELL_SIZE + CELL_BORDER_SIZE) * width + CELL_BORDER_SIZE, j * (CELL_SIZE + CELL_BORDER_SIZE) + CELL_BORDER_SIZE);
    }

    context.stroke();
};

const getIndex = (row, col) => {
    return row * width + col;
}

const drawCells = () => {
    const cellsPtr = universe.cells();
    const cells = new Uint8Array(memory.buffer, cellsPtr, width * height);

    context.beginPath();

    for (let row = 0; row < height; row++) {
        for (let col = 0; col < width; col++) {
            const index = getIndex(row, col);

            context.fillStyle = cells[index] === Cell.Dead ? DEAD_COLOR : ALIVE_COLOR;

            context.fillRect(
                col * (CELL_SIZE + CELL_BORDER_SIZE) + CELL_BORDER_SIZE,
                row * (CELL_SIZE + CELL_BORDER_SIZE) + CELL_BORDER_SIZE,
                CELL_SIZE,
                CELL_SIZE
            );
        }
    }

    context.stroke();
}