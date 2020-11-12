use bevy::prelude::*; // bevy実装
use rand::{thread_rng, Rng}; // ランダム実装
use std::time::Duration; // タイマー実装
use std::fs::File; // ファイル処理実装
use std::io::Read; // ファイル読込実装

fn file_open(path: &str) -> String { // 指定ファイルのテキスト情報を返す
    let mut text = String::new();
    if let Ok(mut data) = File::open(path) { data.read_to_string(&mut text).expect(path); }
    text
}

#[derive(Clone)]
struct Config { // 設定データフォーマット
    sna_x: u32, // ヘビの横幅
    sna_y: u32, // ヘビの縦幅
    sna_s: f32, // ヘビの速度
    win_width: u32, // window横幅
    win_height: u32, // window縦幅
    food_x: u32, // 餌の横幅
    food_y: u32, // 餌の縦幅
    food_pop: u64, // 餌出現頻度
    tail_shrink: bool, // 尾収縮
}

struct Score {
    point: u32, // 食べた餌数
    food: u32, // 残っている餌数
}

fn config_load() -> Config { // 設定データをconfig.txtから読み込んで返す
    let mut x = Vec::new();
    for line in file_open("config.txt").lines() {
        x.push(format!("{}",&line[line.find(':').unwrap()+1..]));
    }
    Config {
        sna_x: x[0].parse().unwrap(),
        sna_y: x[1].parse().unwrap(),
        sna_s: x[2].parse().unwrap(),
        win_width: x[3].parse().unwrap(),
        win_height: x[4].parse().unwrap(),
        food_x: x[5].parse().unwrap(),
        food_y: x[6].parse().unwrap(),
        food_pop: x[7].parse().unwrap(),
        tail_shrink: if x[8]=="true" { true } else { false },
    }
}

struct SnakeHead; // ヘビ頭
struct SnakeSegment; // ヘビ尾
struct Food; // 餌
struct Materials { // マテリアル
    head_material: Handle<ColorMaterial>, // ヘビ頭の色
    segment_material: Handle<ColorMaterial>, // ヘビ尾の色
    food_material: Handle<ColorMaterial>, // 餌の色
}
struct FoodSpawnTimer(Timer); // 餌出現頻度
struct GrowthEvent; // 餌食べイベント
#[derive(Default)]
struct SnakeSegments(Vec<Entity>); // ヘビ尾
#[derive(Default)]
struct LastTailPosition(Option<Transform>); // ヘビ成長
struct GameResetEvent; // ゲームリセット

fn setup(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) { // 初期設定
    commands.spawn(Camera2dComponents::default()); // 2Dカメラ設置
    commands.insert_resource(Materials { // マテリアル設置
        head_material: materials.add(Color::rgb(0.0, 0.2, 0.0).into()), // ヘビ頭の色設定
        segment_material: materials.add(Color::rgb(0.0, 0.3, 0.0).into()), // ヘビ尾の色の設定
        food_material: materials.add(Color::rgb(0.7, 0.1, 0.1).into()), // 餌の色設定
    });
}

fn spawn_snake(
    mut commands: Commands,
    materials: Res<Materials>,
    config: Res<Config>,
    mut segments: ResMut<SnakeSegments>,
) { // ヘビ頭定義
    segments.0 = vec![
        commands
            .spawn(SpriteComponents {
                material: materials.head_material.clone(), // ヘビ頭の色設定(コピー)
                sprite: Sprite::new(Vec2::new(config.sna_x as f32, config.sna_y as f32)), // ヘビ頭のサイズ指定
                ..Default::default()
            })
            .with(SnakeHead) // ヘビ頭タグ
            .with(SnakeSegment) // ヘビ尾タグ
            .current_entity()
            .unwrap(),
        spawn_segment( // 尾表示
            &mut commands,
            &materials.segment_material,
            config,
            Transform::default(),
        ),
    ];
}

fn spawn_segment( // ヘビ尾定義
    commands: &mut Commands,
    material: &Handle<ColorMaterial>,
    config: Res<Config>,
    position: Transform,
) -> Entity {
    commands
        .spawn(SpriteComponents {
            material: material.clone(), // 尾の色(コピー)
            sprite: Sprite::new(Vec2::new((config.sna_x - 2) as f32, (config.sna_y - 2) as f32)), // ヘビ尾のサイズ
            transform: position, // 出現位置
            ..SpriteComponents::default()
        })
        .with(SnakeSegment) // ヘビ尾タグ
        .current_entity()
        .unwrap()
}

fn snake_movement( // ヘビ移動処理
    keyboard_input: Res<Input<KeyCode>>, // キーコード
    config: Res<Config>, // 設定
    segments: ResMut<SnakeSegments>, // ヘビ尾
    mut last_tail_position: ResMut<LastTailPosition>, // ヘビ成長
    mut game_reset_events: ResMut<Events<GameResetEvent>>, // ゲームリセット
    mut heads: Query<(Entity, &mut SnakeHead)>, // ヘビ頭の移動
    mut positions: Query<&mut Transform>, // 位置
) {
    let segment_positions = segments.0.iter().map(|e| *positions.get_mut(*e).unwrap()).collect::<Vec<Transform>>(); // 各要素の位置情報取得
    if let Some((head_entity, mut _head)) = heads.iter_mut().next() {
        let mut head_pos = positions.get_mut(head_entity).unwrap();
        let mut change = false; // 変更check
        let win_x = (config.win_width / 2) as f32; // 配置調整
        let win_y = (config.win_height / 2) as f32; // 配置調整
        if keyboard_input.pressed(KeyCode::A) && head_pos.translation.x() > -win_x { // 左ボタン
            *head_pos.translation.x_mut() -= config.sna_s; change = true; // 速度に応じて移動処理
        }
        if keyboard_input.pressed(KeyCode::D) && head_pos.translation.x() < win_x { // 右ボタン
            *head_pos.translation.x_mut() += config.sna_s; change = true;
        }
        if keyboard_input.pressed(KeyCode::S) && head_pos.translation.y() > -win_y { // 上ボタン
            *head_pos.translation.y_mut() -= config.sna_s; change = true;
        }
        if keyboard_input.pressed(KeyCode::W) && head_pos.translation.y() < win_y { // 下ボタン
            *head_pos.translation.y_mut() += config.sna_s; change = true;
        }
        if keyboard_input.pressed(KeyCode::Space) { // ゲームリセット
            game_reset_events.send(GameResetEvent);
        }
        if change | config.tail_shrink { // ヘビ尾追跡/収縮
            segment_positions.iter().zip(segments.0.iter().skip(1)).for_each(|(pos, segment)| {
            *positions.get_mut(*segment).unwrap() = *pos; });
        }
        last_tail_position.0 = Some(*segment_positions.last().unwrap()); // ヘビ成長
    }
}

fn food_spawner(
    mut commands: Commands,
    materials: Res<Materials>,
    config: Res<Config>,
    time: Res<Time>,
    mut timer: ResMut<FoodSpawnTimer>,
    mut score: ResMut<Score>,
) {
    timer.0.tick(time.delta_seconds);
    if timer.0.finished { // タイマーごとに処理
        let mut rng = thread_rng(); // ランダム値準備
        commands
            .spawn(SpriteComponents {
                material: materials.food_material.clone(), // 餌の色
                sprite: Sprite::new(Vec2::new(config.food_x as f32, config.food_y as f32)), // 餌のサイズ
                transform: Transform {
                    translation: Vec3::new( // ランダム配置
                        rng.gen_range(0,config.win_width) as f32 - (config.win_width / 2) as f32,
                        rng.gen_range(0, config.win_height) as f32 - (config.win_height / 2) as f32,
                         0.0),
                    ..Default::default()
                },
                ..Default::default()
            })
            .with(Food); // 餌タグ
        score.food += 1; // 餌出現カウント
        println!("Score:{} Food:{}", score.point.to_string(), (score.food - score.point).to_string()); // コンソールに表示
    }
}

fn snake_eating( // 餌食べ処理
    mut commands: Commands,
    config: Res<Config>,
    mut growth_events: ResMut<Events<GrowthEvent>>,
    mut score: ResMut<Score>,
    food_positions: Query<With<Food, (Entity, &Transform)>>,
    head_positions: Query<With<SnakeHead, &Transform>>,
) {
    for head_pos in head_positions.iter() {
        for (ent, food_pos) in food_positions.iter() {
            let rangex = ( config.sna_x + config.food_x ) / 2;
            let rangey = ( config.sna_y + config.food_y ) / 2;
            if (food_pos.translation.x() - head_pos.translation.x()).abs() as u32 <= rangex // 当たり判定(隣にくっつけば取得)
            && (food_pos.translation.y() - head_pos.translation.y()).abs() as u32 <= rangey {
                commands.despawn(ent); // 餌削除
                growth_events.send(GrowthEvent); // 尾を増やす
                score.point += 1; // 取得餌数カウント
            }
        }
    }
}

fn snake_growth(
    mut commands: Commands,
    growth_events: Res<Events<GrowthEvent>>,
    last_tail_position: Res<LastTailPosition>,
    config: Res<Config>,
    mut segments: ResMut<SnakeSegments>,
    mut growth_reader: Local<EventReader<GrowthEvent>>,
    materials: Res<Materials>,
) {
    if growth_reader.iter(&growth_events).next().is_some() {
        segments.0.push(spawn_segment( // 最後尾に尾を追加
            &mut commands,
            &materials.segment_material,
            config,
            last_tail_position.0.unwrap(),
        ));
    }
}

fn game_reset(
    mut commands: Commands,
    mut reader: Local<EventReader<GameResetEvent>>,
    game_reset_events: Res<Events<GameResetEvent>>,
    materials: Res<Materials>,
    config: Res<Config>,
    segments_res: ResMut<SnakeSegments>,
    food: Query<With<Food, Entity>>,
    segments: Query<With<SnakeSegment, Entity>>,
) {
    if reader.iter(&game_reset_events).next().is_some() {
        for ent in food.iter().chain(segments.iter()) {
            commands.despawn(ent); // 全餌削除
        }
        spawn_snake(commands, materials, config, segments_res); // ヘビ再表示
    }
}

fn main() {
    let conf = config_load();
    App::build() // アプリ構成
    .add_resource(conf.clone()) // 設定データ追加
    .add_resource(Score { point: 0, food: 0 }) // スコアセット
    .add_resource(ClearColor(Color::rgb(0.05, 0.5, 0.1))) // 背景色
    .add_resource(WindowDescriptor { // window設定
        title: "Snake - AariaToys".to_string(), // タイトル
        width: conf.win_width, // window幅
        height: conf.win_height, // window高さ
        ..Default::default()
    })
    .add_resource(FoodSpawnTimer(Timer::new(Duration::from_millis(conf.food_pop),true))) // 餌出現頻度設定追加
    .add_resource(SnakeSegments::default()) // ヘビ尾追加
    .add_resource(LastTailPosition::default()) // ヘビ成長追加
    .add_event::<GrowthEvent>() // 餌食べイベント追加
    .add_event::<GameResetEvent>() // ゲームリセットイベント追加
    .add_startup_system(setup.system()) // 初期設定追加
    .add_startup_stage("game_setup") // ステージ追加
    .add_startup_system_to_stage("game_setup", spawn_snake.system()) // ヘビ表示追加
    .add_system(snake_movement.system()) // ヘビ移動処理追加
    .add_system(food_spawner.system()) // 餌出現処理追加
    .add_system(snake_eating.system()) // 餌食べ処理追加
    .add_system(snake_growth.system()) // ヘビ成長処理追加
    .add_system(game_reset.system()) // ゲームリセット処理追加 
    .add_plugins(DefaultPlugins) // デフォルト機能追加
    .run(); // アプリ実行
}
