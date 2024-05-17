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
use SCTLR_read__1::*;
use common::*;
pub fn BTypeCompatible_PACIXSP<T: Tracer>(
    state: &mut State,
    tracer: &T,
    gs_15724: (),
) -> bool {
    #[derive(Default)]
    struct FunctionState {
        gs_15725: bool,
        return_value: bool,
        ga_11662: i64,
        ga_11663: ProductType5c790c8ef59cc8b2,
        gs_15724: (),
    }
    let fn_state = FunctionState {
        gs_15724,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_0_0: const #16970u : u32
        let s_0_0: u32 = 16970;
        // D s_0_1: read-reg s_0_0:u8
        let s_0_1: u8 = {
            let value = state.read_register::<u8>(s_0_0 as isize);
            tracer.read_register(s_0_0 as isize, value);
            value
        };
        // D s_0_2: cast zx s_0_1 -> bv
        let s_0_2: Bits = Bits::new(s_0_1 as u128, 2u16);
        // C s_0_3: const #1u : u8
        let s_0_3: u8 = 1;
        // C s_0_4: cast zx s_0_3 -> bv
        let s_0_4: Bits = Bits::new(s_0_3 as u128, 2u16);
        // D s_0_5: cmp-eq s_0_2 s_0_4
        let s_0_5: bool = ((s_0_2) == (s_0_4));
        // N s_0_6: branch s_0_5 b11 b1
        if s_0_5 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_1_0: const #16970u : u32
        let s_1_0: u32 = 16970;
        // D s_1_1: read-reg s_1_0:u8
        let s_1_1: u8 = {
            let value = state.read_register::<u8>(s_1_0 as isize);
            tracer.read_register(s_1_0 as isize, value);
            value
        };
        // D s_1_2: cast zx s_1_1 -> bv
        let s_1_2: Bits = Bits::new(s_1_1 as u128, 2u16);
        // C s_1_3: const #2u : u8
        let s_1_3: u8 = 2;
        // C s_1_4: cast zx s_1_3 -> bv
        let s_1_4: Bits = Bits::new(s_1_3 as u128, 2u16);
        // D s_1_5: cmp-eq s_1_2 s_1_4
        let s_1_5: bool = ((s_1_2) == (s_1_4));
        // D s_1_6: write-var gs#15725 <= s_1_5
        fn_state.gs_15725 = s_1_5;
        // N s_1_7: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_2_0: read-var gs#15725:u8
        let s_2_0: bool = fn_state.gs_15725;
        // N s_2_1: branch s_2_0 b10 b3
        if s_2_0 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_3_0: const #16970u : u32
        let s_3_0: u32 = 16970;
        // D s_3_1: read-reg s_3_0:u8
        let s_3_1: u8 = {
            let value = state.read_register::<u8>(s_3_0 as isize);
            tracer.read_register(s_3_0 as isize, value);
            value
        };
        // D s_3_2: cast zx s_3_1 -> bv
        let s_3_2: Bits = Bits::new(s_3_1 as u128, 2u16);
        // C s_3_3: const #3u : u8
        let s_3_3: u8 = 3;
        // C s_3_4: cast zx s_3_3 -> bv
        let s_3_4: Bits = Bits::new(s_3_3 as u128, 2u16);
        // D s_3_5: cmp-eq s_3_2 s_3_4
        let s_3_5: bool = ((s_3_2) == (s_3_4));
        // N s_3_6: branch s_3_5 b6 b4
        if s_3_5 {
            return block_6(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_4_0: const #0u : u8
        let s_4_0: bool = false;
        // D s_4_1: write-var return_value <= s_4_0
        fn_state.return_value = s_4_0;
        // N s_4_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_5_0: read-var return_value:u8
        let s_5_0: bool = fn_state.return_value;
        // N s_5_1: return s_5_0
        return s_5_0;
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_6_0: const #16975u : u32
        let s_6_0: u32 = 16975;
        // D s_6_1: read-reg s_6_0:u8
        let s_6_1: u8 = {
            let value = state.read_register::<u8>(s_6_0 as isize);
            tracer.read_register(s_6_0 as isize, value);
            value
        };
        // D s_6_2: cast zx s_6_1 -> bv
        let s_6_2: Bits = Bits::new(s_6_1 as u128, 2u16);
        // C s_6_3: const #448u : u32
        let s_6_3: u32 = 448;
        // D s_6_4: read-reg s_6_3:u8
        let s_6_4: u8 = {
            let value = state.read_register::<u8>(s_6_3 as isize);
            tracer.read_register(s_6_3 as isize, value);
            value
        };
        // D s_6_5: cast zx s_6_4 -> bv
        let s_6_5: Bits = Bits::new(s_6_4 as u128, 2u16);
        // D s_6_6: cmp-eq s_6_2 s_6_5
        let s_6_6: bool = ((s_6_2) == (s_6_5));
        // N s_6_7: branch s_6_6 b9 b7
        if s_6_6 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_7_0: const #36s : i64
        let s_7_0: i64 = 36;
        // D s_7_1: write-var ga#11662 <= s_7_0
        fn_state.ga_11662 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_8_0: read-var ga#11662:i64
        let s_8_0: i64 = fn_state.ga_11662;
        // C s_8_1: const #() : ()
        let s_8_1: () = ();
        // S s_8_2: call SCTLR_read__1(s_8_1)
        let s_8_2: ProductType5c790c8ef59cc8b2 = SCTLR_read__1(state, tracer, s_8_1);
        // D s_8_3: write-var ga#11663 <= s_8_2
        fn_state.ga_11663 = s_8_2;
        // D s_8_4: read-var ga#11663.0:struct
        let s_8_4: u64 = fn_state.ga_11663._0;
        // D s_8_5: cast zx s_8_4 -> bv
        let s_8_5: Bits = Bits::new(s_8_4 as u128, 64u16);
        // D s_8_6: cast zx s_8_0 -> i
        let s_8_6: i128 = (i128::try_from(s_8_0).unwrap());
        // C s_8_7: const #1u : u64
        let s_8_7: u64 = 1;
        // D s_8_8: bit-extract s_8_5 s_8_6 s_8_7
        let s_8_8: Bits = (Bits::new(
            ((s_8_5) >> (s_8_6)).value(),
            u16::try_from(s_8_7).unwrap(),
        ));
        // D s_8_9: cast reint s_8_8 -> u8
        let s_8_9: bool = ((s_8_8.value()) != 0);
        // C s_8_10: const #0s : i
        let s_8_10: i128 = 0;
        // C s_8_11: const #0u : u64
        let s_8_11: u64 = 0;
        // D s_8_12: cast zx s_8_9 -> u64
        let s_8_12: u64 = (s_8_9 as u64);
        // C s_8_13: const #1u : u64
        let s_8_13: u64 = 1;
        // D s_8_14: and s_8_12 s_8_13
        let s_8_14: u64 = ((s_8_12) & (s_8_13));
        // D s_8_15: cmp-eq s_8_14 s_8_13
        let s_8_15: bool = ((s_8_14) == (s_8_13));
        // D s_8_16: lsl s_8_12 s_8_10
        let s_8_16: u64 = s_8_12 << s_8_10;
        // D s_8_17: or s_8_11 s_8_16
        let s_8_17: u64 = ((s_8_11) | (s_8_16));
        // D s_8_18: cmpl s_8_16
        let s_8_18: u64 = !s_8_16;
        // D s_8_19: and s_8_11 s_8_18
        let s_8_19: u64 = ((s_8_11) & (s_8_18));
        // D s_8_20: select s_8_15 s_8_17 s_8_19
        let s_8_20: u64 = if s_8_15 { s_8_17 } else { s_8_19 };
        // D s_8_21: cast trunc s_8_20 -> u8
        let s_8_21: bool = ((s_8_20) != 0);
        // D s_8_22: cast zx s_8_21 -> bv
        let s_8_22: Bits = Bits::new(s_8_21 as u128, 1u16);
        // C s_8_23: const #0u : u8
        let s_8_23: bool = false;
        // C s_8_24: cast zx s_8_23 -> bv
        let s_8_24: Bits = Bits::new(s_8_23 as u128, 1u16);
        // D s_8_25: cmp-eq s_8_22 s_8_24
        let s_8_25: bool = ((s_8_22) == (s_8_24));
        // D s_8_26: write-var return_value <= s_8_25
        fn_state.return_value = s_8_25;
        // N s_8_27: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_9_0: const #35s : i64
        let s_9_0: i64 = 35;
        // D s_9_1: write-var ga#11662 <= s_9_0
        fn_state.ga_11662 = s_9_0;
        // N s_9_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_10_0: const #1u : u8
        let s_10_0: bool = true;
        // D s_10_1: write-var return_value <= s_10_0
        fn_state.return_value = s_10_0;
        // N s_10_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_11_0: const #1u : u8
        let s_11_0: bool = true;
        // D s_11_1: write-var gs#15725 <= s_11_0
        fn_state.gs_15725 = s_11_0;
        // N s_11_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
