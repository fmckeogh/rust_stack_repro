#![no_std]
#![allow(non_snake_case)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(unused_parens)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unreachable_code)]
#![allow(unused_doc_comments)]
#![allow(non_upper_case_globals)]
//! BOREALIS GENERATED FILE
extern crate alloc;
use Bit::*;
use SPESetDataVirtualAddress::*;
use SPESetDataPhysicalAddress::*;
use CollectPhysicalAddress::*;
use common::*;
pub fn SPESampleLoadStore<T: Tracer>(
    state: &mut State,
    tracer: &T,
    is_load: bool,
    accdesc: ProductType9878976b5bcce9c9,
    addrdesc: ProductTypece7c66ccb2cab13e,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_20089: bool,
        gs_20079: bool,
        ga_15527: bool,
        gs_20083: bool,
        sample_loads: bool,
        gs_20086: bool,
        gs_20090: bool,
        gs_20077: bool,
        gs_20088: bool,
        gs_20087: bool,
        gs_20078: bool,
        gs_20076: bool,
        sample_stores: bool,
        is_load: bool,
        accdesc: ProductType9878976b5bcce9c9,
        addrdesc: ProductTypece7c66ccb2cab13e,
    }
    let fn_state = FunctionState {
        is_load,
        accdesc,
        addrdesc,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var accdesc.1:struct
        let s_0_0: u32 = fn_state.accdesc._1;
        // C s_0_1: const #10u : u32
        let s_0_1: u32 = 10;
        // D s_0_2: cmp-eq s_0_0 s_0_1
        let s_0_2: bool = ((s_0_0) == (s_0_1));
        // N s_0_3: branch s_0_2 b48 b1
        if s_0_2 {
            return block_48(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var accdesc.1:struct
        let s_1_0: u32 = fn_state.accdesc._1;
        // C s_1_1: const #0u : u32
        let s_1_1: u32 = 0;
        // D s_1_2: cmp-eq s_1_0 s_1_1
        let s_1_2: bool = ((s_1_0) == (s_1_1));
        // N s_1_3: branch s_1_2 b47 b2
        if s_1_2 {
            return block_47(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var accdesc.1:struct
        let s_2_0: u32 = fn_state.accdesc._1;
        // C s_2_1: const #6u : u32
        let s_2_1: u32 = 6;
        // D s_2_2: cmp-eq s_2_0 s_2_1
        let s_2_2: bool = ((s_2_0) == (s_2_1));
        // N s_2_3: branch s_2_2 b46 b3
        if s_2_2 {
            return block_46(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var accdesc.1:struct
        let s_3_0: u32 = fn_state.accdesc._1;
        // C s_3_1: const #13u : u32
        let s_3_1: u32 = 13;
        // D s_3_2: cmp-eq s_3_0 s_3_1
        let s_3_2: bool = ((s_3_0) == (s_3_1));
        // N s_3_3: branch s_3_2 b45 b4
        if s_3_2 {
            return block_45(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var accdesc.1:struct
        let s_4_0: u32 = fn_state.accdesc._1;
        // C s_4_1: const #8u : u32
        let s_4_1: u32 = 8;
        // D s_4_2: cmp-eq s_4_0 s_4_1
        let s_4_2: bool = ((s_4_0) == (s_4_1));
        // D s_4_3: write-var gs#20076 <= s_4_2
        fn_state.gs_20076 = s_4_2;
        // N s_4_4: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var gs#20076:u8
        let s_5_0: bool = fn_state.gs_20076;
        // D s_5_1: write-var gs#20077 <= s_5_0
        fn_state.gs_20077 = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var gs#20077:u8
        let s_6_0: bool = fn_state.gs_20077;
        // D s_6_1: write-var gs#20078 <= s_6_0
        fn_state.gs_20078 = s_6_0;
        // N s_6_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var gs#20078:u8
        let s_7_0: bool = fn_state.gs_20078;
        // D s_7_1: write-var gs#20079 <= s_7_0
        fn_state.gs_20079 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var gs#20079:u8
        let s_8_0: bool = fn_state.gs_20079;
        // N s_8_1: branch s_8_0 b44 b9
        if s_8_0 {
            return block_44(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #0s : i
        let s_9_0: i128 = 0;
        // C s_9_1: const #13528u : u32
        let s_9_1: u32 = 13528;
        // D s_9_2: read-reg s_9_1:u8
        let s_9_2: u8 = {
            let value = state.read_register::<u8>(s_9_1 as isize);
            tracer.read_register(s_9_1 as isize, value);
            value
        };
        // D s_9_3: cast zx s_9_2 -> bv
        let s_9_3: Bits = Bits::new(s_9_2 as u128, 8u16);
        // C s_9_4: const #1u : u64
        let s_9_4: u64 = 1;
        // D s_9_5: bit-extract s_9_3 s_9_0 s_9_4
        let s_9_5: Bits = (Bits::new(
            ((s_9_3) >> (s_9_0)).value(),
            u16::try_from(s_9_4).unwrap(),
        ));
        // D s_9_6: cast reint s_9_5 -> u8
        let s_9_6: bool = ((s_9_5.value()) != 0);
        // C s_9_7: const #0s : i
        let s_9_7: i128 = 0;
        // C s_9_8: const #0u : u64
        let s_9_8: u64 = 0;
        // D s_9_9: cast zx s_9_6 -> u64
        let s_9_9: u64 = (s_9_6 as u64);
        // C s_9_10: const #1u : u64
        let s_9_10: u64 = 1;
        // D s_9_11: and s_9_9 s_9_10
        let s_9_11: u64 = ((s_9_9) & (s_9_10));
        // D s_9_12: cmp-eq s_9_11 s_9_10
        let s_9_12: bool = ((s_9_11) == (s_9_10));
        // D s_9_13: lsl s_9_9 s_9_7
        let s_9_13: u64 = s_9_9 << s_9_7;
        // D s_9_14: or s_9_8 s_9_13
        let s_9_14: u64 = ((s_9_8) | (s_9_13));
        // D s_9_15: cmpl s_9_13
        let s_9_15: u64 = !s_9_13;
        // D s_9_16: and s_9_8 s_9_15
        let s_9_16: u64 = ((s_9_8) & (s_9_15));
        // D s_9_17: select s_9_12 s_9_14 s_9_16
        let s_9_17: u64 = if s_9_12 { s_9_14 } else { s_9_16 };
        // D s_9_18: cast trunc s_9_17 -> u8
        let s_9_18: bool = ((s_9_17) != 0);
        // D s_9_19: cast zx s_9_18 -> bv
        let s_9_19: Bits = Bits::new(s_9_18 as u128, 1u16);
        // C s_9_20: const #0u : u8
        let s_9_20: bool = false;
        // C s_9_21: cast zx s_9_20 -> bv
        let s_9_21: Bits = Bits::new(s_9_20 as u128, 1u16);
        // D s_9_22: cmp-eq s_9_19 s_9_21
        let s_9_22: bool = ((s_9_19) == (s_9_21));
        // N s_9_23: branch s_9_22 b43 b10
        if s_9_22 {
            return block_43(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #0u : u8
        let s_10_0: bool = false;
        // D s_10_1: write-var gs#20083 <= s_10_0
        fn_state.gs_20083 = s_10_0;
        // N s_10_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var gs#20083:u8
        let s_11_0: bool = fn_state.gs_20083;
        // D s_11_1: write-var sample_loads <= s_11_0
        fn_state.sample_loads = s_11_0;
        // C s_11_2: const #0s : i
        let s_11_2: i128 = 0;
        // C s_11_3: const #13528u : u32
        let s_11_3: u32 = 13528;
        // D s_11_4: read-reg s_11_3:u8
        let s_11_4: u8 = {
            let value = state.read_register::<u8>(s_11_3 as isize);
            tracer.read_register(s_11_3 as isize, value);
            value
        };
        // D s_11_5: cast zx s_11_4 -> bv
        let s_11_5: Bits = Bits::new(s_11_4 as u128, 8u16);
        // C s_11_6: const #1u : u64
        let s_11_6: u64 = 1;
        // D s_11_7: bit-extract s_11_5 s_11_2 s_11_6
        let s_11_7: Bits = (Bits::new(
            ((s_11_5) >> (s_11_2)).value(),
            u16::try_from(s_11_6).unwrap(),
        ));
        // D s_11_8: cast reint s_11_7 -> u8
        let s_11_8: bool = ((s_11_7.value()) != 0);
        // C s_11_9: const #0s : i
        let s_11_9: i128 = 0;
        // C s_11_10: const #0u : u64
        let s_11_10: u64 = 0;
        // D s_11_11: cast zx s_11_8 -> u64
        let s_11_11: u64 = (s_11_8 as u64);
        // C s_11_12: const #1u : u64
        let s_11_12: u64 = 1;
        // D s_11_13: and s_11_11 s_11_12
        let s_11_13: u64 = ((s_11_11) & (s_11_12));
        // D s_11_14: cmp-eq s_11_13 s_11_12
        let s_11_14: bool = ((s_11_13) == (s_11_12));
        // D s_11_15: lsl s_11_11 s_11_9
        let s_11_15: u64 = s_11_11 << s_11_9;
        // D s_11_16: or s_11_10 s_11_15
        let s_11_16: u64 = ((s_11_10) | (s_11_15));
        // D s_11_17: cmpl s_11_15
        let s_11_17: u64 = !s_11_15;
        // D s_11_18: and s_11_10 s_11_17
        let s_11_18: u64 = ((s_11_10) & (s_11_17));
        // D s_11_19: select s_11_14 s_11_16 s_11_18
        let s_11_19: u64 = if s_11_14 { s_11_16 } else { s_11_18 };
        // D s_11_20: cast trunc s_11_19 -> u8
        let s_11_20: bool = ((s_11_19) != 0);
        // D s_11_21: cast zx s_11_20 -> bv
        let s_11_21: Bits = Bits::new(s_11_20 as u128, 1u16);
        // C s_11_22: const #1u : u8
        let s_11_22: bool = true;
        // C s_11_23: cast zx s_11_22 -> bv
        let s_11_23: Bits = Bits::new(s_11_22 as u128, 1u16);
        // D s_11_24: cmp-eq s_11_21 s_11_23
        let s_11_24: bool = ((s_11_21) == (s_11_23));
        // N s_11_25: branch s_11_24 b42 b12
        if s_11_24 {
            return block_42(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #0u : u8
        let s_12_0: bool = false;
        // D s_12_1: write-var gs#20086 <= s_12_0
        fn_state.gs_20086 = s_12_0;
        // N s_12_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var gs#20086:u8
        let s_13_0: bool = fn_state.gs_20086;
        // D s_13_1: write-var sample_stores <= s_13_0
        fn_state.sample_stores = s_13_0;
        // C s_13_2: const #11528u : u32
        let s_13_2: u32 = 11528;
        // D s_13_3: read-reg s_13_2:u8
        let s_13_3: bool = {
            let value = state.read_register::<bool>(s_13_2 as isize);
            tracer.read_register(s_13_2 as isize, value);
            value
        };
        // D s_13_4: not s_13_3
        let s_13_4: bool = !s_13_3;
        // N s_13_5: branch s_13_4 b41 b14
        if s_13_4 {
            return block_41(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var sample_loads:u8
        let s_14_0: bool = fn_state.sample_loads;
        // N s_14_1: branch s_14_0 b40 b15
        if s_14_0 {
            return block_40(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #0u : u8
        let s_15_0: bool = false;
        // D s_15_1: write-var gs#20087 <= s_15_0
        fn_state.gs_20087 = s_15_0;
        // N s_15_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var gs#20087:u8
        let s_16_0: bool = fn_state.gs_20087;
        // D s_16_1: write-var gs#20088 <= s_16_0
        fn_state.gs_20088 = s_16_0;
        // N s_16_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var gs#20088:u8
        let s_17_0: bool = fn_state.gs_20088;
        // N s_17_1: branch s_17_0 b39 b18
        if s_17_0 {
            return block_39(state, tracer, fn_state);
        } else {
            return block_18(state, tracer, fn_state);
        };
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var sample_stores:u8
        let s_18_0: bool = fn_state.sample_stores;
        // N s_18_1: branch s_18_0 b38 b19
        if s_18_0 {
            return block_38(state, tracer, fn_state);
        } else {
            return block_19(state, tracer, fn_state);
        };
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_19_0: const #0u : u8
        let s_19_0: bool = false;
        // D s_19_1: write-var gs#20089 <= s_19_0
        fn_state.gs_20089 = s_19_0;
        // N s_19_2: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_20_0: read-var gs#20089:u8
        let s_20_0: bool = fn_state.gs_20089;
        // D s_20_1: write-var gs#20090 <= s_20_0
        fn_state.gs_20090 = s_20_0;
        // N s_20_2: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var gs#20090:u8
        let s_21_0: bool = fn_state.gs_20090;
        // N s_21_1: branch s_21_0 b34 b22
        if s_21_0 {
            return block_34(state, tracer, fn_state);
        } else {
            return block_22(state, tracer, fn_state);
        };
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_22_0: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_23_0: const #11528u : u32
        let s_23_0: u32 = 11528;
        // D s_23_1: read-reg s_23_0:u8
        let s_23_1: bool = {
            let value = state.read_register::<bool>(s_23_0 as isize);
            tracer.read_register(s_23_0 as isize, value);
            value
        };
        // D s_23_2: not s_23_1
        let s_23_2: bool = !s_23_1;
        // N s_23_3: branch s_23_2 b25 b24
        if s_23_2 {
            return block_25(state, tracer, fn_state);
        } else {
            return block_24(state, tracer, fn_state);
        };
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_24_0: return
        return;
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_25_0: const #1u : u8
        let s_25_0: u8 = 1;
        // C s_25_1: const #17136u : u32
        let s_25_1: u32 = 17136;
        // N s_25_2: write-reg s_25_1 <= s_25_0
        let s_25_2: () = {
            state.write_register::<u8>(s_25_1 as isize, s_25_0);
            tracer.write_register(s_25_1 as isize, s_25_0);
        };
        // C s_25_3: const #1u : u8
        let s_25_3: bool = true;
        // C s_25_4: const #11528u : u32
        let s_25_4: u32 = 11528;
        // N s_25_5: write-reg s_25_4 <= s_25_3
        let s_25_5: () = {
            state.write_register::<bool>(s_25_4 as isize, s_25_3);
            tracer.write_register(s_25_4 as isize, s_25_3);
        };
        // C s_25_6: const #1s : i
        let s_25_6: i128 = 1;
        // C s_25_7: const #13528u : u32
        let s_25_7: u32 = 13528;
        // D s_25_8: read-reg s_25_7:u8
        let s_25_8: u8 = {
            let value = state.read_register::<u8>(s_25_7 as isize);
            tracer.read_register(s_25_7 as isize, value);
            value
        };
        // D s_25_9: cast zx s_25_8 -> bv
        let s_25_9: Bits = Bits::new(s_25_8 as u128, 8u16);
        // C s_25_10: const #8u : u8
        let s_25_10: u8 = 8;
        // C s_25_11: cast zx s_25_10 -> bv
        let s_25_11: Bits = Bits::new(s_25_10 as u128, 7u16);
        // C s_25_12: const #6s : i
        let s_25_12: i128 = 6;
        // C s_25_13: const #1u : u64
        let s_25_13: u64 = 1;
        // C s_25_14: cast zx s_25_13 -> bv
        let s_25_14: Bits = Bits::new(s_25_13 as u128, 64u16);
        // C s_25_15: lsl s_25_14 s_25_12
        let s_25_15: Bits = s_25_14 << s_25_12;
        // C s_25_16: sub s_25_15 s_25_14
        let s_25_16: Bits = ((s_25_15) - (s_25_14));
        // C s_25_17: and s_25_11 s_25_16
        let s_25_17: Bits = ((s_25_11) & (s_25_16));
        // C s_25_18: lsl s_25_17 s_25_6
        let s_25_18: Bits = s_25_17 << s_25_6;
        // C s_25_19: lsl s_25_16 s_25_6
        let s_25_19: Bits = s_25_16 << s_25_6;
        // C s_25_20: cmpl s_25_19
        let s_25_20: Bits = !s_25_19;
        // D s_25_21: and s_25_9 s_25_20
        let s_25_21: Bits = ((s_25_9) & (s_25_20));
        // D s_25_22: or s_25_21 s_25_18
        let s_25_22: Bits = ((s_25_21) | (s_25_18));
        // D s_25_23: cast reint s_25_22 -> u8
        let s_25_23: u8 = (s_25_22.value() as u8);
        // C s_25_24: const #13528u : u32
        let s_25_24: u32 = 13528;
        // N s_25_25: write-reg s_25_24 <= s_25_23
        let s_25_25: () = {
            state.write_register::<u8>(s_25_24 as isize, s_25_23);
            tracer.write_register(s_25_24 as isize, s_25_23);
        };
        // D s_25_26: read-var is_load:u8
        let s_25_26: bool = fn_state.is_load;
        // N s_25_27: branch s_25_26 b33 b26
        if s_25_26 {
            return block_33(state, tracer, fn_state);
        } else {
            return block_26(state, tracer, fn_state);
        };
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_26_0: const #1u : u8
        let s_26_0: bool = true;
        // D s_26_1: write-var ga#15527 <= s_26_0
        fn_state.ga_15527 = s_26_0;
        // N s_26_2: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_27_0: read-var ga#15527:u8
        let s_27_0: bool = fn_state.ga_15527;
        // D s_27_1: call Bit(s_27_0)
        let s_27_1: bool = Bit(state, tracer, s_27_0);
        // C s_27_2: const #0s : i
        let s_27_2: i128 = 0;
        // C s_27_3: const #13528u : u32
        let s_27_3: u32 = 13528;
        // D s_27_4: read-reg s_27_3:u8
        let s_27_4: u8 = {
            let value = state.read_register::<u8>(s_27_3 as isize);
            tracer.read_register(s_27_3 as isize, value);
            value
        };
        // D s_27_5: cast zx s_27_4 -> bv
        let s_27_5: Bits = Bits::new(s_27_4 as u128, 8u16);
        // C s_27_6: const #1u : u64
        let s_27_6: u64 = 1;
        // D s_27_7: bit-insert s_27_5 s_27_5 s_27_2 s_27_6
        let s_27_7: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_27_6 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_27_5.length(),
            );
            (s_27_5 & mask) | (s_27_5 << s_27_2)
        };
        // D s_27_8: cast reint s_27_7 -> u8
        let s_27_8: u8 = (s_27_7.value() as u8);
        // C s_27_9: const #13528u : u32
        let s_27_9: u32 = 13528;
        // N s_27_10: write-reg s_27_9 <= s_27_8
        let s_27_10: () = {
            state.write_register::<u8>(s_27_9 as isize, s_27_8);
            tracer.write_register(s_27_9 as isize, s_27_8);
        };
        // D s_27_11: read-var is_load:u8
        let s_27_11: bool = fn_state.is_load;
        // N s_27_12: branch s_27_11 b32 b28
        if s_27_11 {
            return block_32(state, tracer, fn_state);
        } else {
            return block_28(state, tracer, fn_state);
        };
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_28_0: const #1u : u32
        let s_28_0: u32 = 1;
        // C s_28_1: const #19040u : u32
        let s_28_1: u32 = 19040;
        // N s_28_2: write-reg s_28_1 <= s_28_0
        let s_28_2: () = {
            state.write_register::<u32>(s_28_1 as isize, s_28_0);
            tracer.write_register(s_28_1 as isize, s_28_0);
        };
        // N s_28_3: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_29_0: read-var accdesc.1:struct
        let s_29_0: u32 = fn_state.accdesc._1;
        // C s_29_1: const #9u : u32
        let s_29_1: u32 = 9;
        // D s_29_2: cmp-eq s_29_0 s_29_1
        let s_29_2: bool = ((s_29_0) == (s_29_1));
        // N s_29_3: branch s_29_2 b31 b30
        if s_29_2 {
            return block_31(state, tracer, fn_state);
        } else {
            return block_30(state, tracer, fn_state);
        };
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_30_0: return
        return;
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_31_0: const #1s : i
        let s_31_0: i128 = 1;
        // C s_31_1: const #13528u : u32
        let s_31_1: u32 = 13528;
        // D s_31_2: read-reg s_31_1:u8
        let s_31_2: u8 = {
            let value = state.read_register::<u8>(s_31_1 as isize);
            tracer.read_register(s_31_1 as isize, value);
            value
        };
        // D s_31_3: cast zx s_31_2 -> bv
        let s_31_3: Bits = Bits::new(s_31_2 as u128, 8u16);
        // C s_31_4: const #24u : u8
        let s_31_4: u8 = 24;
        // C s_31_5: cast zx s_31_4 -> bv
        let s_31_5: Bits = Bits::new(s_31_4 as u128, 7u16);
        // C s_31_6: const #6s : i
        let s_31_6: i128 = 6;
        // C s_31_7: const #1u : u64
        let s_31_7: u64 = 1;
        // C s_31_8: cast zx s_31_7 -> bv
        let s_31_8: Bits = Bits::new(s_31_7 as u128, 64u16);
        // C s_31_9: lsl s_31_8 s_31_6
        let s_31_9: Bits = s_31_8 << s_31_6;
        // C s_31_10: sub s_31_9 s_31_8
        let s_31_10: Bits = ((s_31_9) - (s_31_8));
        // C s_31_11: and s_31_5 s_31_10
        let s_31_11: Bits = ((s_31_5) & (s_31_10));
        // C s_31_12: lsl s_31_11 s_31_0
        let s_31_12: Bits = s_31_11 << s_31_0;
        // C s_31_13: lsl s_31_10 s_31_0
        let s_31_13: Bits = s_31_10 << s_31_0;
        // C s_31_14: cmpl s_31_13
        let s_31_14: Bits = !s_31_13;
        // D s_31_15: and s_31_3 s_31_14
        let s_31_15: Bits = ((s_31_3) & (s_31_14));
        // D s_31_16: or s_31_15 s_31_12
        let s_31_16: Bits = ((s_31_15) | (s_31_12));
        // D s_31_17: cast reint s_31_16 -> u8
        let s_31_17: u8 = (s_31_16.value() as u8);
        // C s_31_18: const #13528u : u32
        let s_31_18: u32 = 13528;
        // N s_31_19: write-reg s_31_18 <= s_31_17
        let s_31_19: () = {
            state.write_register::<u8>(s_31_18 as isize, s_31_17);
            tracer.write_register(s_31_18 as isize, s_31_17);
        };
        // C s_31_20: const #1u : u8
        let s_31_20: bool = true;
        // C s_31_21: const #10528u : u32
        let s_31_21: u32 = 10528;
        // N s_31_22: write-reg s_31_21 <= s_31_20
        let s_31_22: () = {
            state.write_register::<bool>(s_31_21 as isize, s_31_20);
            tracer.write_register(s_31_21 as isize, s_31_20);
        };
        // N s_31_23: return
        return;
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_32_0: const #0u : u32
        let s_32_0: u32 = 0;
        // C s_32_1: const #19040u : u32
        let s_32_1: u32 = 19040;
        // N s_32_2: write-reg s_32_1 <= s_32_0
        let s_32_2: () = {
            state.write_register::<u32>(s_32_1 as isize, s_32_0);
            tracer.write_register(s_32_1 as isize, s_32_0);
        };
        // N s_32_3: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_33_0: const #0u : u8
        let s_33_0: bool = false;
        // D s_33_1: write-var ga#15527 <= s_33_0
        fn_state.ga_15527 = s_33_0;
        // N s_33_2: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_34_0: read-var addrdesc.7:struct
        let s_34_0: u64 = fn_state.addrdesc._7;
        // D s_34_1: call SPESetDataVirtualAddress(s_34_0)
        let s_34_1: () = SPESetDataVirtualAddress(state, tracer, s_34_0);
        // C s_34_2: const #() : ()
        let s_34_2: () = ();
        // S s_34_3: call CollectPhysicalAddress(s_34_2)
        let s_34_3: bool = CollectPhysicalAddress(state, tracer, s_34_2);
        // N s_34_4: branch s_34_3 b37 b35
        if s_34_3 {
            return block_37(state, tracer, fn_state);
        } else {
            return block_35(state, tracer, fn_state);
        };
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_35_0: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_36_0: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_37_0: read-var addrdesc:struct
        let s_37_0: ProductTypece7c66ccb2cab13e = fn_state.addrdesc;
        // D s_37_1: read-var accdesc:struct
        let s_37_1: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_37_2: call SPESetDataPhysicalAddress(s_37_0, s_37_1)
        let s_37_2: () = SPESetDataPhysicalAddress(state, tracer, s_37_0, s_37_1);
        // N s_37_3: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_38_0: read-var is_load:u8
        let s_38_0: bool = fn_state.is_load;
        // D s_38_1: not s_38_0
        let s_38_1: bool = !s_38_0;
        // D s_38_2: write-var gs#20089 <= s_38_1
        fn_state.gs_20089 = s_38_1;
        // N s_38_3: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_39_0: const #1u : u8
        let s_39_0: bool = true;
        // D s_39_1: write-var gs#20090 <= s_39_0
        fn_state.gs_20090 = s_39_0;
        // N s_39_2: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_40_0: read-var is_load:u8
        let s_40_0: bool = fn_state.is_load;
        // D s_40_1: write-var gs#20087 <= s_40_0
        fn_state.gs_20087 = s_40_0;
        // N s_40_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_41_0: const #1u : u8
        let s_41_0: bool = true;
        // D s_41_1: write-var gs#20088 <= s_41_0
        fn_state.gs_20088 = s_41_0;
        // N s_41_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_42_0: const #11528u : u32
        let s_42_0: u32 = 11528;
        // D s_42_1: read-reg s_42_0:u8
        let s_42_1: bool = {
            let value = state.read_register::<bool>(s_42_0 as isize);
            tracer.read_register(s_42_0 as isize, value);
            value
        };
        // D s_42_2: write-var gs#20086 <= s_42_1
        fn_state.gs_20086 = s_42_1;
        // N s_42_3: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_43_0: const #11528u : u32
        let s_43_0: u32 = 11528;
        // D s_43_1: read-reg s_43_0:u8
        let s_43_1: bool = {
            let value = state.read_register::<bool>(s_43_0 as isize);
            tracer.read_register(s_43_0 as isize, value);
            value
        };
        // D s_43_2: write-var gs#20083 <= s_43_1
        fn_state.gs_20083 = s_43_1;
        // N s_43_3: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_44_0: return
        return;
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_45_0: const #1u : u8
        let s_45_0: bool = true;
        // D s_45_1: write-var gs#20076 <= s_45_0
        fn_state.gs_20076 = s_45_0;
        // N s_45_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_46_0: const #1u : u8
        let s_46_0: bool = true;
        // D s_46_1: write-var gs#20077 <= s_46_0
        fn_state.gs_20077 = s_46_0;
        // N s_46_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_47_0: const #1u : u8
        let s_47_0: bool = true;
        // D s_47_1: write-var gs#20078 <= s_47_0
        fn_state.gs_20078 = s_47_0;
        // N s_47_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_48_0: const #1u : u8
        let s_48_0: bool = true;
        // D s_48_1: write-var gs#20079 <= s_48_0
        fn_state.gs_20079 = s_48_0;
        // N s_48_2: jump b8
        return block_8(state, tracer, fn_state);
    }
}
