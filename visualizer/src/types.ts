export type VisualizerSettingInfo = {
    input: string;
    output: string;
    err: string;
    seed: number;
    turn: number;
    maxTurn: number;
    problemId: string;
};

export type VisualizerResult = {
    svgString: string;
    err: string;
    score: number;
};
