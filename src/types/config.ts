export interface Config {
    player: PlayerConfig;
    interface: InterfaceConfig;
}

export interface PlayerConfig {
    executable: string;
    args: string[];
}

export interface InterfaceConfig {
    theme: string;
    language: string;
    grid_size: number;
}