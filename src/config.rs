use crate::toml_seek::Seek;
use crate::Error;
use crate::ErrorKind;
use std::collections::HashMap;
use std::env;
use std::fs;
use toml;
use toml::Value;

pub struct Scheme {
    pub preset_name: Option<String>,
    pub show_progress: Option<bool>,
    pub show_info: Option<bool>,
    pub threads_count: Option<i32>,
    pub simulate_terminal: Option<bool>,
    pub repeat_count: Option<i32>,
    // pub stdin_type: Option<String>,
    pub stdout_type: Option<String>,
    pub stdout_file_path: Option<String>,
    pub stderr_type: Option<String>,
    pub stderr_file_path: Option<String>,
    pub program: Option<String>,
    pub args_template: Option<String>,
    pub args_switches: Option<String>,
    pub input_list: Option<Vec<String>>, // Priority: config file's "input.list" > process args > "input_dir_path" and others
    pub input_dir_path: Option<String>,
    pub input_dir_deep: Option<bool>,
    pub output_file_path: Option<String>, // Almost like "input_list"
    pub output_file_overwrite: Option<bool>,
    pub output_dir_path: Option<String>,
    pub output_dir_keep_struct: Option<bool>,
    pub output_file_name_extension: Option<String>,
    pub output_file_name_prefix: Option<String>,
    pub output_file_name_suffix: Option<String>,
}

impl Scheme {
    fn complement(&mut self, default: &Self) {
        fn complement_option<T: Clone>(target: &mut Option<T>, source: &Option<T>) {
            if let Some(v) = source {
                target.get_or_insert(v.clone());
            }
        }

        // TODO: Use macro instead?
        complement_option(&mut self.preset_name, &default.preset_name); // Unnecessary?
        complement_option(&mut self.show_progress, &default.show_progress);
        complement_option(&mut self.show_info, &default.show_info);
        complement_option(&mut self.threads_count, &default.threads_count);
        complement_option(&mut self.simulate_terminal, &default.simulate_terminal);
        complement_option(&mut self.repeat_count, &default.repeat_count);
        complement_option(&mut self.stdout_type, &default.stdout_type);
        complement_option(&mut self.stdout_file_path, &default.stdout_file_path);
        complement_option(&mut self.stderr_type, &default.stderr_type);
        complement_option(&mut self.stderr_file_path, &default.stderr_file_path);
        complement_option(&mut self.program, &default.program);
        complement_option(&mut self.args_template, &default.args_template);
        complement_option(&mut self.args_switches, &default.args_switches);
        complement_option(&mut self.input_list, &default.input_list);
        complement_option(&mut self.input_dir_path, &default.input_dir_path);
        complement_option(&mut self.input_dir_deep, &default.input_dir_deep);
        complement_option(&mut self.output_file_path, &default.output_file_path);
        complement_option(
            &mut self.output_file_overwrite,
            &default.output_file_overwrite,
        );
        complement_option(&mut self.output_dir_path, &default.output_dir_path);
        complement_option(
            &mut self.output_dir_keep_struct,
            &default.output_dir_keep_struct,
        );
        complement_option(
            &mut self.output_file_name_extension,
            &default.output_file_name_extension,
        );
        complement_option(
            &mut self.output_file_name_prefix,
            &default.output_file_name_prefix,
        );
        complement_option(
            &mut self.output_file_name_suffix,
            &default.output_file_name_suffix,
        );
    }

    fn from_toml_value(v: &Value) -> Self {
        Self {
            preset_name: v.seek_str("preset"),
            show_progress: v.seek_bool("message.progress"),
            show_info: v.seek_bool("message.info"),
            threads_count: v.seek_i32("threads.count"),
            simulate_terminal: v.seek_bool("process.simulate_terminal"),
            repeat_count: v.seek_i32("repeat.count"),
            stdout_type: v.seek_str("stdio.stdout.type"),
            stdout_file_path: v.seek_str("stdio.stdout.file.path"),
            stderr_type: v.seek_str("stdio.stderr.type"),
            stderr_file_path: v.seek_str("stdio.stderr.file.path"),
            program: v.seek_str("program"),
            args_template: v.seek_str("args.template"),
            args_switches: v.seek_str("args.switches"),
            input_list: v.seek_vec_str("input.list"),
            input_dir_path: v.seek_str("input.dir.path"),
            input_dir_deep: v.seek_bool("input.dir.deep"),
            output_file_path: v.seek_str("output.file.path"),
            output_file_overwrite: v.seek_bool("output.file.overwrite"),
            output_dir_path: v.seek_str("output.dir.path"),
            output_dir_keep_struct: v.seek_bool("output.dir.keep_struct"),
            output_file_name_extension: v.seek_str("output.file_name.extension"),
            output_file_name_prefix: v.seek_str("output.file_name.prefix"),
            output_file_name_suffix: v.seek_str("output.file_name.suffix"),
        }
    }
}

pub struct Config {
    pub presets: HashMap<String, Scheme>,
    pub orders: Vec<Scheme>,
}

impl Config {
    pub fn new() -> Result<Self, Error> {
        let mut file_path = env::current_exe().unwrap();

        file_path.set_extension("toml");
        if let Ok(toml_str) = fs::read_to_string(&file_path) {
            return Config::from_toml(toml_str);
        }

        // file_path.set_extension("json");
        // if let Ok(json_str) = fs::read_to_string(&file_path) {
        //     return Config::from_json(json_str);
        // }

        Err(Error {
            kind: ErrorKind::ConfigFileCanNotRead,
            inner: None,
            message: None,
        })
    }

    pub fn _from_toml_test() -> Result<Self, Error> {
        Self::from_toml(String::from(
            r#"
            [global]
            message.progress = true
            message.info = true
            webui.ip = '127.0.0.1'
            webui.port = 9090

            [presets.default]
            threads.count = 4
            threads.stop_painc = false # TODO
            process.simulate_terminal = false # TODO
            repeat.count = 1
            stdio.stdout.type = 'file' # ignore | normal | file
            stdio.stdout.file.path = './target/test-stdout.log'
            stdio.stderr.type = 'file'
            stdio.stderr.file.path = './target/test-stderr.log'

            [presets.cwebp]
            program = 'cwebp.exe'
            args.template = '{args.switches} {input.file_path} -o {output.file_path}' # TODO: trope "{{" to real "{"
            args.switches = '-m 6'
            output.file_name.extension = 'webp'

            [presets.cwebp_lossless]
            preset = 'cwebp'
            args.switches = '-lossless -m 6 -noalpha -sharp_yuv -metadata none'

            [presets.clock]
            repeat.count = 10
            program = 'cmd'
            args.template = '/c echo {args.switches} ; {repeat.index} ; {repeat.position} && timeout /t 1 > nul'
            args.switches = 'time: %time%'
            threads.count = 1
            stdio.stdout.type = 'normal'
            stdio.stderr.type = 'normal'

            [presets.timeout]
            program = 'timeout'
            args.template = '{args.switches}'
            args.switches = '/t 5'
            threads.count = 1
            stdio.stdout.type = 'normal'
            stdio.stderr.type = 'normal'

            [[orders]]
            preset = 'cwebp_lossless'
            program = 'D:\Library\libwebp\libwebp_1.0.0\bin\cwebp.exe'
            input.dir.path = 'D:\Temp\foundry_test\source'
            output.dir.path = 'D:\Temp\foundry_test\target'
            output.file_name.prefix = 'out_'
            output.file_name.suffix = '_out'
            "#,
        ))
    }

    fn from_toml(toml_str: String) -> Result<Self, Error> {
        let toml_value: Value = toml_str.parse().or_else(|e| {
            Err(Error {
                kind: ErrorKind::ConfigTomlIllegal,
                inner: Some(Box::new(e)),
                message: None,
            })
        })?;
        let mut cfg = Self {
            presets: HashMap::new(),
            orders: Vec::new(),
        };

        for (name, toml_value) in toml_value.get("presets").unwrap().as_table().unwrap() {
            let preset = Scheme::from_toml_value(toml_value);
            cfg.presets.insert(name.clone(), preset);
        }
        for toml_value in toml_value.get("orders").unwrap().as_array().unwrap() {
            let order = Scheme::from_toml_value(toml_value);
            cfg.orders.push(order);
        }
        Self::fix(&mut cfg)?;
        Ok(cfg)
    }

    // fn from_json() -> Config {}

    fn fix(cfg: &mut Self) -> Result<(), Error> {
        fn inherit_fill(
            order: &mut Scheme,
            current_preset_name: &str,
            presets: &HashMap<String, Scheme>,
            stack_deep: i32,
        ) {
            if stack_deep > 64 {
                return;
            }
            if let Some(preset) = presets.get(current_preset_name) {
                order.complement(preset);
                if let Some(ref next_preset_name) = preset.preset_name {
                    inherit_fill(order, next_preset_name, presets, stack_deep + 1);
                }
            }
        }

        let mut args = env::args();
        args.next();
        let mut input_list = Vec::new();
        let mut output_file_path = String::new();
        let mut is_output_item = false;
        for arg in args {
            if is_output_item {
                if output_file_path.is_empty() {
                    output_file_path = arg;
                } else {
                    return Err(Error {
                        kind: ErrorKind::ConfigIllegal,
                        inner: None,
                        message: Some(String::from("too many output path in process arguments")),
                    });
                }
            } else if arg.starts_with('-') {
                if arg == "-o" || arg == "--output" {
                    is_output_item = true;
                } else {
                    return Err(Error {
                        kind: ErrorKind::ConfigIllegal,
                        inner: None,
                        message: Some(format!("unknown switch `{}` in process arguments", arg)),
                    });
                }
            } else {
                input_list.push(arg);
            }
        }

        for order in &mut cfg.orders {
            if let Some(preset_name) = &order.preset_name {
                let first_preset_name = preset_name.clone();
                inherit_fill(order, &first_preset_name, &cfg.presets, 1);
            }
            // You can use [default] -> [other preset you want]
            inherit_fill(order, "default", &cfg.presets, 1);
            // But can not [build_in] -> [other preset]
            order.complement(&Scheme {
                preset_name: Some(String::from("build_in")),
                show_progress: Some(true),
                show_info: Some(true),
                threads_count: Some(1),
                simulate_terminal: Some(false),
                repeat_count: Some(1),
                stdout_type: Some(String::from("ignore")),
                stdout_file_path: None,
                stderr_type: Some(String::from("ignore")),
                stderr_file_path: None,
                program: None,
                args_template: Some(String::new()),
                args_switches: Some(String::new()),
                input_list: if input_list.is_empty() {
                    None
                } else {
                    Some(input_list.to_owned())
                },
                input_dir_path: None,
                input_dir_deep: Some(false),
                output_file_path: if output_file_path.is_empty() {
                    None
                } else {
                    Some(output_file_path.to_owned())
                },
                output_file_overwrite: Some(true),
                output_dir_path: None,
                output_dir_keep_struct: Some(false),
                output_file_name_extension: None,
                output_file_name_prefix: None,
                output_file_name_suffix: None,
            });
        }

        if cfg.orders.is_empty() {
            Err(Error {
                kind: ErrorKind::ConfigIllegal,
                inner: None,
                message: Some(String::from("order not provided")),
            })
        } else {
            Ok(())
        }
    }
}
