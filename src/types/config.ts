export interface Config {
    player: PlayerConfig;
    interface: InterfaceConfig;
    vn: VnConfig;
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

export interface VnConfig {
    textractor_executable: string;
    textractor_enabled: boolean;
    texthooker_enabled: boolean;
}