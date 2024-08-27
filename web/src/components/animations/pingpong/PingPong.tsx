"use client"

import React, { useEffect, useState } from 'react';

const PaddleSize = {
    width: 10,
    height: 80,
};

const BallSize = {
    diameter: 10,
};

enum PlayerType {
    User = 'user',
    AI = 'ai',
}


const useGameLoop = (
    canvasWidth,
    canvasHeight,
    player,
    aiController,
    currentPlayer,
    setPlayer,
    setBall,
    resettingRef
) => {
    const frameIdRef = useRef(null);

    const gameLoop = useCallback(() => {
        const { ball, playerController } = window;
        ball.update();

        // Update player and AI based on the current player
        if (currentPlayer === PlayerType.User) {
            playerController?.update(playerController.upKey, playerController.downKey);
        } else {
            aiController.update();
        }

        // Swap players if the ball goes off the canvas
        if (
            ball.position.x < 0 ||
            ball.position.x + BallSize.diameter > canvasWidth
        ) {
            currentPlayer === PlayerType.User
                ? setCurrentPlayer(PlayerType.AI)
                : setCurrentPlayer(PlayerType.User);
        }

        // Detect scoring and reset the ball's position with a 1-second delay
        if (ball.position.x < 0) {
            // The player scores
            setPlayer((prevPlayer) => ({ ...prevPlayer, score: prevPlayer.score + 1 }));
            resettingRef.current = true;

            // Wait for 1 second before resetting the ball and unset the resetting flag
            setTimeout(() => {
                resettingRef.current = false;
                ball.reset();
                frameIdRef.current = requestAnimationFrame(gameLoop);
            }, 1000);
        } else if (ball.position.x + BallSize.diameter > canvasWidth) {
            // The AI scores (you may want to handle this differently)
            aiController.offsetScore(+1);
            resettingRef.current = true;

            setTimeout(() => {
                resettingRef.current = false;
                ball.reset();
                frameIdRef.current = requestAnimationFrame(gameLoop);
            }, 1000);
        } else {
            frameIdRef.current = requestAnimationFrame(gameLoop);
        }
    }, [canvasWidth, currentPlayer, aiController, player, setPlayer]);

    useEffect(() => {
        if (frameIdRef.current === null) {
            frameIdRef.current = requestAnimationFrame(gameLoop);
        }

        return () => {
            if (frameIdRef.current) {
                cancelAnimationFrame(frameIdRef.current);
            }
        };
    }, [gameLoop]);

    return frameIdRef;
};


class Ball {
    position = { x: 0, y: 0 };
    velocity = { x: 0, y: 0 };

    constructor(private canvasWidth: number, private canvasHeight: number) {
        this.reset();
    }

    reset = () => {
        this.position = {
            x: this.canvasWidth / 2 - BallSize.diameter / 2,
            y: this.canvasHeight / 2 - BallSize.diameter / 2,
        };

        const angle = Math.random() * 2 * Math.PI;
        const speed = 5;
        this.velocity = {
            x: Math.cos(angle) * speed,
            y: Math.sin(angle) * speed,
        };
    };

    update = () => {
        this.position.x += this.velocity.x;
        this.position.y += this.velocity.y;

        if (
            this.position.x < 0 ||
            this.position.x + BallSize.diameter > this.canvasWidth
        ) {
            this.velocity.x *= -1;
        }

        if (
            this.position.y < 0 ||
            this.position.y + BallSize.diameter > this.canvasHeight
        ) {
            this.velocity.y *= -1;
        }
    };
}

class Player {
    position = { x: 0, y: 0 };
    score = 0;

    constructor(private canvasHeight: number) {
        this.position = {
            x: PaddleSize.width,
            y: canvasHeight / 2 - PaddleSize.height / 2,
        };
    }

    update = (upKey: boolean, downKey: boolean) => {
        if (upKey && this.position.y > 0) {
            this.position.y -= 5;
        } else if (downKey && this.position.y + PaddleSize.height < this.canvasHeight) {
            this.position.y += 5;
        }
    };
}

class AIController {
    // score = 0;

    score = 0;

    constructor(private canvasHeight: number, private ball: Ball, private player: Player) {

    }

    update = () => {
        const ballCenterY = this.ball.position.y + BallSize.diameter / 2;
        const paddleCenterY = this.player.position.y + PaddleSize.height / 2;

        if (ballCenterY < paddleCenterY && this.player.position.y > 0) {
            this.player.position.y -= 3;
        } else if (ballCenterY > paddleCenterY && this.player.position.y + PaddleSize.height < this.canvasHeight) {
            this.player.position.y += 3;
        }
    };

    getScore = () => {
        return this.score;
    }

    offsetScore = (offset: number) => {
        this.score += offset;
    }
}

class PlayerController {
    upKey = false;
    downKey = false;

    constructor() {
        window.addEventListener('keydown', this.handleKeyDown);
        window.addEventListener('keyup', this.handleKeyUp);
    }

    handleKeyDown = (event: KeyboardEvent) => {
        if (event.key === 'ArrowUp') {
            this.upKey = true;
        } else if (event.key === 'ArrowDown') {
            this.downKey = true;
        }
    };

    handleKeyUp = (event: KeyboardEvent) => {
        if (event.key === 'ArrowUp') {
            this.upKey = false;
        } else if (event.key === 'ArrowDown') {
            this.downKey = false;
        }
    };

    cleanup = () => {
        window.removeEventListener('keydown', this.handleKeyDown);
        window.removeEventListener('keyup', this.handleKeyUp);
    };

    update = (upKey: boolean, downKey: boolean) => {
        this.upKey = upKey;
        this.downKey = downKey;
    };
}

const PingPong: React.FC = () => {
    const canvasWidth = 900;
    const canvasHeight = 100;

    const [ball, setBall] = useState(() => new Ball(canvasWidth, canvasHeight));
    const [player, setPlayer] = useState(() => new Player(canvasHeight));
    const [aiController] = useState(() => new AIController(canvasHeight, ball, player));
    const [playerController, setPlayerController] = useState<PlayerController | null>(null);
    const [currentPlayer, setCurrentPlayer] = useState<PlayerType>(PlayerType.AI);

    const [frameId, setFrameId] = useState<number | null>(null);

    const render = () => {
        const canvas = document.getElementById('canvas') as HTMLCanvasElement;
        const context = canvas.getContext('2d')!;

        context.clearRect(0, 0, canvasWidth, canvasHeight);

        context.fillStyle = 'white';
        context.fillRect(
            player.position.x,
            player.position.y,
            PaddleSize.width,
            PaddleSize.height
        );

        context.fillRect(
            canvasWidth - PaddleSize.width * 2,
            player.position.y,
            PaddleSize.width,
            PaddleSize.height
        );

        context.fillRect(
            ball.position.x,
            ball.position.y,
            BallSize.diameter,
            BallSize.diameter
        );

        context.font = '30px Arial';
        context.fillText(player.score.toString(), 100, 50);
        context.fillText(aiController.getScore().toString(), canvasWidth - 100, 50);

        setFrameId(requestAnimationFrame(render)); // Request next frame
    };
    const [resetting, setResetting] = useState(false); // State to track if the ball is resetting

    const gameLoop = () => {
        ball.update();

        // Update player and AI based on the current player
        if (currentPlayer === PlayerType.User) {
            playerController?.update(playerController.upKey, playerController.downKey);
        } else {
            aiController.update();
        }

        // Swap players if the ball goes off the canvas
        if (
            ball.position.x < 0 ||
            ball.position.x + BallSize.diameter > canvasWidth
        ) {
            setCurrentPlayer(
                currentPlayer === PlayerType.User ? PlayerType.AI : PlayerType.User
            );
        }

        // Detect scoring and reset the ball's position with a 1-second delay
        if (ball.position.x < 0) {
            // The player scores
            player.score += 1;

            // Reset the ball and set the resetting flag to true
            setBall(new Ball(canvasWidth, canvasHeight));
            setResetting(true);

            // Wait for 1 second before resetting the ball and unset the resetting flag
            setTimeout(() => {
                setResetting(false);
                ball.reset();
                setFrameId(requestAnimationFrame(gameLoop)); // Request next frame after the delay
            }, 1000);
        } else if (ball.position.x + BallSize.diameter > canvasWidth) {
            // The AI scores (you may want to handle this differently)
            aiController.offsetScore(+1);
            setBall(new Ball(canvasWidth, canvasHeight));
            setTimeout(() => {
                setResetting(false);
                ball.reset();
                setFrameId(requestAnimationFrame(gameLoop));
            }, 1000);
        } else {
            setFrameId(requestAnimationFrame(gameLoop)); // Request next frame if no scoring happens
        }
    };

    useEffect(() => {
        if (frameId === null) {
            setFrameId(requestAnimationFrame(render)); // Start rendering loop
            setFrameId(requestAnimationFrame(gameLoop)); // Start game loop
        }
    }, [frameId]);


    return (
        <div style={{
            height: '5vh',
            padding: '10px',
        }}>
            <canvas id="canvas" width={canvasWidth} height={canvasHeight} />
        </div>
    );
}

export default PingPong;
