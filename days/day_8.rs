use std::i32::MAX;

pub fn solve(lines: Vec<String>) -> (i32, i32) {
    let sol1 = solution1(&lines, 25, 6);
    let sol2 = solution2(&lines);
    (sol1, sol2)
}

fn parse_input(line: &String, width: usize, height: usize) -> Vec<Vec<i32>> {
    let pixels_in_layer = width * height;
    let mut layers = vec![Vec::with_capacity(pixels_in_layer); line.len()/pixels_in_layer];
    for (i, char) in line.chars().enumerate() {
        layers[i / pixels_in_layer].push(char.to_string().parse::<i32>().unwrap());
    }

    layers
}


fn solution1(lines: &Vec<String>, width: usize, height: usize) -> i32 {
    let mut layer_with_fewest_zeros = 0;
    let mut fewest_zeros = MAX as usize;

    let layers = parse_input(&lines[0], width, height);
    for (i, layer) in layers.iter().enumerate() {
        let zeros = layer.iter().filter(|&digit| *digit == 0).count();
        if zeros < fewest_zeros {
            layer_with_fewest_zeros = i;
            fewest_zeros = zeros;
        }
    }

    let layer = &layers[layer_with_fewest_zeros];
    let res = layer.iter().filter(|&digit| *digit == 1).count() *
        layer.iter().filter(|&digit| *digit == 2).count();
    res as i32
}

fn print_decoded_image(image: Vec<i32>, width: usize, _height: usize) {
    for (i, pixel) in image.into_iter().enumerate() {
        if i % width == 0 {
            println!();
        }
        match pixel {
            0 => { print!(" "); },
            1 => { print!("0"); },
            2 => { print!(" "); },
            _ => unreachable!(),
        }
    }
    println!()
}

fn solution2(lines: &Vec<String>) -> i32 {
    let width = 25;
    let height = 6;
    let layers = parse_input(&lines[0], width, height);
    let mut decoded_image = vec![2; width*height];
    for (i, pixel) in decoded_image.iter_mut().enumerate() {
        let mut cur_layer = 0;
        while layers[cur_layer][i] == 2 && cur_layer < layers.len() {
            cur_layer += 1;
        }
        *pixel = layers[cur_layer][i];
    }

    print_decoded_image(decoded_image, width, height);
    0
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn task1_example() {
        let input = "001222000012011221111122";
        assert_eq!(8, solution1(&vec![input.to_string()], 3, 2));
    }
}