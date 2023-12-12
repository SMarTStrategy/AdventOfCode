use std::convert::TryInto;



fn main() {

    let seeds = "1778931867 1436999653 3684516104 2759374 1192793053 358764985 1698790056 76369598 3733854793 214008036 4054174000 171202266 3630057255 25954395 798587440 316327323 290129780 7039123 3334326492 246125391";
    //     let seeds = "79 14 55 13";
//     let seeds_to_soil = normalize_the_map("50 98 2
// 52 50 48");
//
//     let soil_to_fertilizer = normalize_the_map("0 15 37
// 37 52 2
// 39 0 15");
//
//     let fertilizer_to_water = normalize_the_map("49 53 8
// 0 11 42
// 42 0 7
// 57 7 4");
//     let water_to_light = normalize_the_map("88 18 7
// 18 25 70");
//     let light_to_temperature = normalize_the_map("45 77 23
// 81 45 19
// 68 64 13");
//     let temperature_to_humidity = normalize_the_map("0 69 1
// 1 0 69");
//     let humidity_to_location = normalize_the_map("60 56 37
// 56 93 4");
//
    let seeds_to_soil = normalize_the_map("1965922922 2387203602 59808406
2540447436 434094583 220346698
2217992666 1677013102 149631368
0 700424909 25332775
2488189883 199146916 52257553
1096820417 2512808179 247985955
25332775 725757684 113904366
4167057552 3534307691 127909744
1787863383 0 33562512
2947958449 3662217435 64182733
2907785302 3360301224 40173147
3774943096 4218385602 76581694
693455216 1273647901 403365201
380961654 1909017232 171283127
139237141 1031923388 241724513
2367624034 251404469 38193087
3038180364 3429533867 104773824
2484064707 1826644470 4125176
1344806372 128789319 70357597
3012141182 3109705711 26039182
1821425895 289597556 144497027
598228409 33562512 95226807
2405817121 1830769646 78247586
552244781 654441281 45983628
3641222276 3255639900 104661324
3851524790 3726400168 315532762
2025731328 839662050 192261338
3464769604 4041932930 176452672
1480960140 2080300359 306903243
1415163969 2447012008 65796171
3142954188 2787890295 321815416
3745883600 3400474371 29059496
2787890295 3135744893 119895007");

    let soil_to_fertilizer = normalize_the_map("974611207 822914672 41736646
1617020803 484683369 227984726
2936246728 1897199618 22236339
1599589242 1541299272 17431561
897092117 484593057 90312
2958483067 3614284126 70951194
3636585470 2567424345 5451325
0 2712467888 73845937
2654331234 1997195625 281915494
3241974258 3685235320 31499686
3642036795 3512208003 74698211
1092039278 1347003830 194295442
790747225 1558730833 106344892
897182429 42121154 77428778
1845005529 380881656 29170315
313046856 3156899567 355308436
1952497963 410051971 74541086
3043555623 2369005710 198418635
2040295599 119549932 261331724
3630375808 2279111119 6209662
3383720521 1665075725 217219287
2301627323 2670771541 41696347
1515904313 2285320781 83684929
73845937 1076160024 239200919
2343323670 864651318 182031257
2027039049 3601027576 13256550
2622688347 1315360943 31642887
2525354927 2572875670 19573752
1286334720 1076118575 41449
4007771416 3743620838 287195880
668355292 0 42121154
1286376169 1882295012 14904606
3600939808 1046682575 29436000
2544928679 1919435957 77759668
3273473944 712668095 110246577
3029434261 3586906214 14121362
3743620838 4030816718 264150578
710476446 2786313825 80270779
1016347853 3081208142 75691425
1874175844 2592449422 78322119
1301280775 2866584604 214623538");

    let fertilizer_to_water = normalize_the_map("2256462238 272868806 222756596
2883874475 1945255196 178320531
1025753868 1262393928 220069640
2780673998 2532762404 101990486
222030751 1026223684 236170244
828766276 1895857025 35481787
3755107810 3521770115 3468846
2048518620 2246637941 74827173
550785854 2716185082 5650779
1819399513 252684903 20183903
4134370427 3177062502 147573242
1893363918 2365324803 45336497
458200995 495625402 92584859
3472537931 3557968177 35060837
122101104 2962265359 99929647
2192091098 2172374449 64371140
3538267850 3324635744 184110744
3758576656 3169743127 7319375
0 2410661300 122101104
4281943669 3508746488 13023627
1849504229 2321465114 43859689
3722378594 3525238961 32729216
1245823508 588210261 161392539
878164447 2814675938 147589421
864248063 1931338812 13916384
1938700415 749602800 109818205
2123345793 2745930633 68745305
3765896031 3826310364 218680395
747334084 2634752890 78853599
3984576426 4075659841 149794001
2882664484 1482463568 1209991
723239312 2721835861 24094772
3169743127 3593029014 233281350
1839583416 252656442 28461
2479218834 0 252656442
3507598768 4044990759 30669082
826187683 2713606489 2578593
1839611877 2236745589 9892352
556436633 859421005 166802679
3403024477 4225453842 69513454
2731875276 2123575727 48798722
1407216047 1483673559 412183466");
    let water_to_light = normalize_the_map("2717406339 2056643664 131336656
2149066749 0 164219220
3368552624 3554596203 347071357
648177882 2187980320 58382041
2313285969 387274540 87651626
1956244903 880459831 74655597
630498803 2299352502 17679079
2030900500 1938477415 118166249
2958998106 3901667560 109063910
0 2317031581 376289577
1862852214 809822482 58007210
3962172007 3368255348 186340855
1920859424 164219220 35385479
829057433 474926166 334896316
2848742995 867829692 12630139
3068062016 3067764740 300490608
2628314327 1849385403 89092012
1363807094 955115428 499045120
441461908 1660348508 189036895
1195755118 2693321158 168051976
706559923 199604699 122497510
3824390615 4010731470 137781392
2400937595 2246362361 21188772
3715623981 2958998106 108766634
1163953749 2267551133 31801369
2422126367 1454160548 206187960
376289577 322102209 65172331");
    let light_to_temperature = normalize_the_map("356025838 1142133666 189347695
155231063 1824123064 78066821
3486401291 3916497965 63138309
3054537751 3004205340 53510430
545373533 1910851890 39759630
43898462 1414278470 16090003
233297884 608643392 7876607
0 1051586050 43898462
3108048181 3669777676 160948090
3567273965 3979636274 194976344
146569058 1902189885 8662005
1423483364 1430368473 110560056
2195994218 3468690397 201087279
2497560353 2680093671 17356840
2667148760 3057715770 80634162
2747782922 2697450511 306754829
585133163 1540928529 283194535
1534043420 440260576 168382816
3762250309 2063919656 446944788
3318335889 3300624995 168065402
1308588274 1331481361 82797109
74539638 616519999 1422739
2546794082 4174612618 120354678
3268996271 3201674123 49339618
2127243847 2611343300 68750371
59988465 1095484512 14551173
2397081497 2510864444 100478856
1391385383 1110035685 32097981
1702426236 1016254966 35331084
3549539600 3282890630 17734365
2063919656 3138349932 63324191
1737757320 803400766 212854200
2514917193 3251013741 31876889
4209195097 3830725766 85772199
1003769840 135442142 304818434
241174491 617942738 114851347
868327698 0 135442142
75962377 732794085 70606681");
    let temperature_to_humidity = normalize_the_map("3056037605 2829211160 523334807
321779731 0 47068359
3924298564 3372771457 370668732
1103457901 562085848 156691500
0 240306117 321779731
368848090 1020196358 239953043
2829211160 3743440189 226826445
608801133 718777348 301419010
3579372412 3970266634 324700662
3915140668 3363613561 9157896
910220143 232146746 8159371
3904073074 3352545967 11067594
918379514 47068359 185078387");
    let humidity_to_location = normalize_the_map("1384411009 3878276792 140553103
3206048776 3137400006 12882465
2370337851 2414914902 179202838
23738616 0 161914533
0 262282387 23738616
840681798 3768356904 109919888
2340008493 3054171079 26130417
1524964112 840681798 815044381
3130048499 2668879895 76000277
185653149 548908967 81490523
3465042209 3687267183 62255927
367511526 286021003 262887964
3005762489 2744880172 124286010
1025363841 2357768227 57146675
2549540689 1901546427 456221800
3218931241 4018829895 240309425
1101344310 2869166182 185004897
1286349207 1655726179 98061802
2366138910 1753787981 4198941
3459240666 4259139320 5801543
950601686 2594117740 74762155
3584396646 1757986922 143559505
267143672 161914533 100367854
3527298136 3080301496 57098510
1082510516 3749523110 18833794
3727956151 3150282471 536984712");

    let mut pos: Vec<i64> = Vec::new();

    let seeds_map: Vec<i64>  = seeds.trim().split(" ").collect::<Vec<&str>>().into_iter().map(|x| {
        return x.parse().unwrap();
    }).collect();

    for seed_index in 0..(seeds_map.len()) {

        if !(seed_index % 2 == 0) {
            continue;
        }
        let limit = seeds_map[seed_index] + seeds_map[seed_index + 1];
        pos.push(get_min_location_from_range(seeds_map[seed_index],limit, limit,
                                             seeds_to_soil.clone(),
                                             soil_to_fertilizer.clone(),
                                             fertilizer_to_water.clone(),
                                             water_to_light.clone(),
                                             light_to_temperature.clone(),
                                             temperature_to_humidity.clone(),
                                             humidity_to_location.clone()));
        println!("Worked seed index {}", seed_index);


    }
    let min = pos.iter().fold(pos[0], |mut acc, x| {
        acc = if acc > *x {*x} else {acc};
        return acc;
    } );
    println!("{:?}", min);


}


fn get_min_location_from_range(start: i64, limit: i64, mut current: i64,
                               seeds_to_soil: Vec<[i64;3]>,
                               soil_to_fertilizer: Vec<[i64;3]>,
                               fertilizer_to_water: Vec<[i64;3]>,
                               water_to_light: Vec<[i64;3]>,
                               light_to_temperature: Vec<[i64;3]>,
                               temperature_to_humidity: Vec<[i64;3]>,
                               humidity_to_location: Vec<[i64;3]>) -> i64  {
    // println!("SIZE => {}", mem::size_of_val(seeds_to_soil));
    println!("{} {}", start, limit);
    for seed in start..limit {
        // let now = Instant::now();
        let location = find_location_from_seed(seed,
                                                   &seeds_to_soil,
                                                   &soil_to_fertilizer,
                                                   &fertilizer_to_water,
                                                   &water_to_light,
                                                   &light_to_temperature,
                                                   &temperature_to_humidity,
                                                   &humidity_to_location);
        // let elapsed_time = now.elapsed();
        // println!("Running slow_function() took {} nanos.", elapsed_time.as_nanos());

        if current > location {
            current = location;
        }
    }




    return current
}

fn find_location_from_seed(seed: i64,
                           seeds_to_soil: &Vec<[i64;3]>,
                           soil_to_fertilizer: &Vec<[i64;3]>,
                           fertilizer_to_water: &Vec<[i64;3]>,
                           water_to_light: &Vec<[i64;3]>,
                           light_to_temperature: &Vec<[i64;3]>,
                           temperature_to_humidity: &Vec<[i64;3]>,
                           humidity_to_location: &Vec<[i64;3]>
) -> i64 {





    // print!("{:?} ", pos_humidity_source);
    cycle_on_maps(humidity_to_location,
                  cycle_on_maps(temperature_to_humidity,
                                cycle_on_maps(light_to_temperature,
                                              cycle_on_maps(water_to_light,
                                                            cycle_on_maps(fertilizer_to_water,
                                                                          cycle_on_maps(soil_to_fertilizer,
                                                                                        cycle_on_maps(seeds_to_soil, seed)))))))
}

fn cycle_on_maps(maps: &Vec<[i64;3]>, value_to_check: i64) -> i64 {
    for map in maps {
        let index = get_position_of_source(map[1], map[2], map[0], value_to_check);
        if index == -1 {
            continue;
        }
        return index;
    }

    return value_to_check;

}

fn get_position_of_source(map_source_start: i64, maps_source_range_length: i64, map_position_start: i64, value_to_check: i64) -> i64 {

    let index = get_index_in_range(map_source_start, maps_source_range_length, value_to_check);
    if index == -1 {
        return -1;
    }
    return map_position_start + index;
}

fn get_index_in_range(start: i64, range_length: i64, value_to_check: i64) -> i64 {

    if value_to_check >= start && value_to_check < start + range_length  {
        return value_to_check - start;
    }

    return -1;
}



fn normalize_the_map(map: &str) -> Vec<[i64; 3]> {
    return map.split("\n").collect::<Vec<&str>>().iter().map(|map_object| {
        let vec_map = map_object.trim().split(" ").collect::<Vec<&str>>().iter().map(|map_number| {
            match map_number.parse::<i64>()  {
                Ok(num) => num,
                _ => -1
            }
        }).collect::<Vec<i64>>();
        let res: [i64; 3] = vec_map.try_into().unwrap();
        res
    }).collect()
}
