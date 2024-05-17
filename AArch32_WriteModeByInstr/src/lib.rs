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
use u_get_HCR_Type_TGE::*;
use u_get_SCR_Type_NS::*;
use ELFromM32::*;
use HCR_read::*;
use AArch32_WriteMode::*;
use common::*;
pub fn AArch32_WriteModeByInstr<T: Tracer>(
    state: &mut State,
    tracer: &T,
    mode: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        ga_24489: ProductTypea5cc8de4daab131c,
        gs_31444: bool,
        gs_31443: bool,
        valid_name: bool,
        gs_31445: bool,
        gs_31441: bool,
        gs_31440: bool,
        gs_31442: bool,
        el: u8,
        mode: u8,
    }
    let fn_state = FunctionState {
        mode,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var mode:u8
        let s_0_0: u8 = fn_state.mode;
        // D s_0_1: call ELFromM32(s_0_0)
        let s_0_1: ProductTypea5cc8de4daab131c = ELFromM32(state, tracer, s_0_0);
        // D s_0_2: write-var ga#24489 <= s_0_1
        fn_state.ga_24489 = s_0_1;
        // D s_0_3: read-var ga#24489.0:struct
        let s_0_3: bool = fn_state.ga_24489._0;
        // D s_0_4: read-var ga#24489.1:struct
        let s_0_4: u8 = fn_state.ga_24489._1;
        // D s_0_5: write-var valid_name <= s_0_3
        fn_state.valid_name = s_0_3;
        // D s_0_6: write-var el <= s_0_4
        fn_state.el = s_0_4;
        // D s_0_7: read-var el:u8
        let s_0_7: u8 = fn_state.el;
        // D s_0_8: cast zx s_0_7 -> bv
        let s_0_8: Bits = Bits::new(s_0_7 as u128, 2u16);
        // D s_0_9: cast zx s_0_8 -> i
        let s_0_9: i128 = (s_0_8.value() as i128);
        // D s_0_10: cast reint s_0_9 -> i64
        let s_0_10: i64 = (s_0_9 as i64);
        // C s_0_11: const #16975u : u32
        let s_0_11: u32 = 16975;
        // D s_0_12: read-reg s_0_11:u8
        let s_0_12: u8 = {
            let value = state.read_register::<u8>(s_0_11 as isize);
            tracer.read_register(s_0_11 as isize, value);
            value
        };
        // D s_0_13: cast zx s_0_12 -> bv
        let s_0_13: Bits = Bits::new(s_0_12 as u128, 2u16);
        // D s_0_14: cast zx s_0_13 -> i
        let s_0_14: i128 = (s_0_13.value() as i128);
        // D s_0_15: cast reint s_0_14 -> i64
        let s_0_15: i64 = (s_0_14 as i64);
        // D s_0_16: cast zx s_0_10 -> i
        let s_0_16: i128 = (i128::try_from(s_0_10).unwrap());
        // D s_0_17: cast zx s_0_15 -> i
        let s_0_17: i128 = (i128::try_from(s_0_15).unwrap());
        // D s_0_18: cmp-gt s_0_16 s_0_17
        let s_0_18: bool = ((s_0_16) > (s_0_17));
        // N s_0_19: branch s_0_18 b29 b1
        if s_0_18 {
            return block_29(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_1_0: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #16983u : u32
        let s_2_0: u32 = 16983;
        // D s_2_1: read-reg s_2_0:u8
        let s_2_1: u8 = {
            let value = state.read_register::<u8>(s_2_0 as isize);
            tracer.read_register(s_2_0 as isize, value);
            value
        };
        // D s_2_2: cast zx s_2_1 -> bv
        let s_2_2: Bits = Bits::new(s_2_1 as u128, 5u16);
        // C s_2_3: const #400u : u32
        let s_2_3: u32 = 400;
        // D s_2_4: read-reg s_2_3:u8
        let s_2_4: u8 = {
            let value = state.read_register::<u8>(s_2_3 as isize);
            tracer.read_register(s_2_3 as isize, value);
            value
        };
        // D s_2_5: cast zx s_2_4 -> bv
        let s_2_5: Bits = Bits::new(s_2_4 as u128, 5u16);
        // D s_2_6: cmp-eq s_2_2 s_2_5
        let s_2_6: bool = ((s_2_2) == (s_2_5));
        // N s_2_7: branch s_2_6 b28 b3
        if s_2_6 {
            return block_28(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var mode:u8
        let s_3_0: u8 = fn_state.mode;
        // D s_3_1: cast zx s_3_0 -> bv
        let s_3_1: Bits = Bits::new(s_3_0 as u128, 5u16);
        // C s_3_2: const #400u : u32
        let s_3_2: u32 = 400;
        // D s_3_3: read-reg s_3_2:u8
        let s_3_3: u8 = {
            let value = state.read_register::<u8>(s_3_2 as isize);
            tracer.read_register(s_3_2 as isize, value);
            value
        };
        // D s_3_4: cast zx s_3_3 -> bv
        let s_3_4: Bits = Bits::new(s_3_3 as u128, 5u16);
        // D s_3_5: cmp-eq s_3_1 s_3_4
        let s_3_5: bool = ((s_3_1) == (s_3_4));
        // D s_3_6: write-var gs#31440 <= s_3_5
        fn_state.gs_31440 = s_3_5;
        // N s_3_7: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#31440:u8
        let s_4_0: bool = fn_state.gs_31440;
        // N s_4_1: branch s_4_0 b27 b5
        if s_4_0 {
            return block_27(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #0u : u8
        let s_5_0: bool = false;
        // D s_5_1: write-var gs#31441 <= s_5_0
        fn_state.gs_31441 = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var gs#31441:u8
        let s_6_0: bool = fn_state.gs_31441;
        // N s_6_1: branch s_6_0 b26 b7
        if s_6_0 {
            return block_26(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_7_0: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #16983u : u32
        let s_8_0: u32 = 16983;
        // D s_8_1: read-reg s_8_0:u8
        let s_8_1: u8 = {
            let value = state.read_register::<u8>(s_8_0 as isize);
            tracer.read_register(s_8_0 as isize, value);
            value
        };
        // D s_8_2: cast zx s_8_1 -> bv
        let s_8_2: Bits = Bits::new(s_8_1 as u128, 5u16);
        // C s_8_3: const #384u : u32
        let s_8_3: u32 = 384;
        // D s_8_4: read-reg s_8_3:u8
        let s_8_4: u8 = {
            let value = state.read_register::<u8>(s_8_3 as isize);
            tracer.read_register(s_8_3 as isize, value);
            value
        };
        // D s_8_5: cast zx s_8_4 -> bv
        let s_8_5: Bits = Bits::new(s_8_4 as u128, 5u16);
        // D s_8_6: cmp-eq s_8_2 s_8_5
        let s_8_6: bool = ((s_8_2) == (s_8_5));
        // N s_8_7: branch s_8_6 b25 b9
        if s_8_6 {
            return block_25(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #0u : u8
        let s_9_0: bool = false;
        // D s_9_1: write-var gs#31442 <= s_9_0
        fn_state.gs_31442 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#31442:u8
        let s_10_0: bool = fn_state.gs_31442;
        // N s_10_1: branch s_10_0 b24 b11
        if s_10_0 {
            return block_24(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #0u : u8
        let s_11_0: bool = false;
        // D s_11_1: write-var gs#31443 <= s_11_0
        fn_state.gs_31443 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var gs#31443:u8
        let s_12_0: bool = fn_state.gs_31443;
        // N s_12_1: branch s_12_0 b23 b13
        if s_12_0 {
            return block_23(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #0u : u8
        let s_13_0: bool = false;
        // D s_13_1: write-var gs#31444 <= s_13_0
        fn_state.gs_31444 = s_13_0;
        // N s_13_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var gs#31444:u8
        let s_14_0: bool = fn_state.gs_31444;
        // N s_14_1: branch s_14_0 b22 b15
        if s_14_0 {
            return block_22(state, tracer, fn_state);
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
        // D s_15_1: write-var gs#31445 <= s_15_0
        fn_state.gs_31445 = s_15_0;
        // N s_15_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var gs#31445:u8
        let s_16_0: bool = fn_state.gs_31445;
        // N s_16_1: branch s_16_0 b21 b17
        if s_16_0 {
            return block_21(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_17_0: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var valid_name:u8
        let s_18_0: bool = fn_state.valid_name;
        // D s_18_1: not s_18_0
        let s_18_1: bool = !s_18_0;
        // N s_18_2: branch s_18_1 b20 b19
        if s_18_1 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_19(state, tracer, fn_state);
        };
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var mode:u8
        let s_19_0: u8 = fn_state.mode;
        // D s_19_1: call AArch32_WriteMode(s_19_0)
        let s_19_1: () = AArch32_WriteMode(state, tracer, s_19_0);
        // N s_19_2: return
        return;
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_20_0: const #1u : u8
        let s_20_0: bool = true;
        // C s_20_1: const #16980u : u32
        let s_20_1: u32 = 16980;
        // N s_20_2: write-reg s_20_1 <= s_20_0
        let s_20_2: () = {
            state.write_register::<bool>(s_20_1 as isize, s_20_0);
            tracer.write_register(s_20_1 as isize, s_20_0);
        };
        // N s_20_3: return
        return;
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_21_0: const #0u : u8
        let s_21_0: bool = false;
        // D s_21_1: write-var valid_name <= s_21_0
        fn_state.valid_name = s_21_0;
        // N s_21_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_22_0: const #() : ()
        let s_22_0: () = ();
        // S s_22_1: call HCR_read(s_22_0)
        let s_22_1: ProductType700c18a878c5601b = HCR_read(state, tracer, s_22_0);
        // S s_22_2: call _get_HCR_Type_TGE(s_22_1)
        let s_22_2: bool = u_get_HCR_Type_TGE(state, tracer, s_22_1);
        // S s_22_3: cast zx s_22_2 -> bv
        let s_22_3: Bits = Bits::new(s_22_2 as u128, 1u16);
        // C s_22_4: const #1u : u8
        let s_22_4: bool = true;
        // C s_22_5: cast zx s_22_4 -> bv
        let s_22_5: Bits = Bits::new(s_22_4 as u128, 1u16);
        // S s_22_6: cmp-eq s_22_3 s_22_5
        let s_22_6: bool = ((s_22_3) == (s_22_5));
        // D s_22_7: write-var gs#31445 <= s_22_6
        fn_state.gs_31445 = s_22_6;
        // N s_22_8: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_23_0: const #20920u : u32
        let s_23_0: u32 = 20920;
        // D s_23_1: read-reg s_23_0:struct
        let s_23_1: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_23_0 as isize);
            tracer.read_register(s_23_0 as isize, value);
            value
        };
        // D s_23_2: call _get_SCR_Type_NS(s_23_1)
        let s_23_2: bool = u_get_SCR_Type_NS(state, tracer, s_23_1);
        // D s_23_3: cast zx s_23_2 -> bv
        let s_23_3: Bits = Bits::new(s_23_2 as u128, 1u16);
        // C s_23_4: const #1u : u8
        let s_23_4: bool = true;
        // C s_23_5: cast zx s_23_4 -> bv
        let s_23_5: Bits = Bits::new(s_23_4 as u128, 1u16);
        // D s_23_6: cmp-eq s_23_3 s_23_5
        let s_23_6: bool = ((s_23_3) == (s_23_5));
        // D s_23_7: write-var gs#31444 <= s_23_6
        fn_state.gs_31444 = s_23_6;
        // N s_23_8: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_24_0: read-var el:u8
        let s_24_0: u8 = fn_state.el;
        // D s_24_1: cast zx s_24_0 -> bv
        let s_24_1: Bits = Bits::new(s_24_0 as u128, 2u16);
        // C s_24_2: const #440u : u32
        let s_24_2: u32 = 440;
        // D s_24_3: read-reg s_24_2:u8
        let s_24_3: u8 = {
            let value = state.read_register::<u8>(s_24_2 as isize);
            tracer.read_register(s_24_2 as isize, value);
            value
        };
        // D s_24_4: cast zx s_24_3 -> bv
        let s_24_4: Bits = Bits::new(s_24_3 as u128, 2u16);
        // D s_24_5: cmp-eq s_24_1 s_24_4
        let s_24_5: bool = ((s_24_1) == (s_24_4));
        // D s_24_6: write-var gs#31443 <= s_24_5
        fn_state.gs_31443 = s_24_5;
        // N s_24_7: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_25_0: const #432u : u32
        let s_25_0: u32 = 432;
        // D s_25_1: read-reg s_25_0:u8
        let s_25_1: u8 = {
            let value = state.read_register::<u8>(s_25_0 as isize);
            tracer.read_register(s_25_0 as isize, value);
            value
        };
        // C s_25_2: const #2u : u8
        let s_25_2: u8 = 2;
        // D s_25_3: cmp-lt s_25_1 s_25_2
        let s_25_3: bool = ((s_25_1) < (s_25_2));
        // D s_25_4: write-var gs#31442 <= s_25_3
        fn_state.gs_31442 = s_25_3;
        // N s_25_5: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_26_0: const #0u : u8
        let s_26_0: bool = false;
        // D s_26_1: write-var valid_name <= s_26_0
        fn_state.valid_name = s_26_0;
        // N s_26_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_27_0: const #16983u : u32
        let s_27_0: u32 = 16983;
        // D s_27_1: read-reg s_27_0:u8
        let s_27_1: u8 = {
            let value = state.read_register::<u8>(s_27_0 as isize);
            tracer.read_register(s_27_0 as isize, value);
            value
        };
        // D s_27_2: cast zx s_27_1 -> bv
        let s_27_2: Bits = Bits::new(s_27_1 as u128, 5u16);
        // D s_27_3: read-var mode:u8
        let s_27_3: u8 = fn_state.mode;
        // D s_27_4: cast zx s_27_3 -> bv
        let s_27_4: Bits = Bits::new(s_27_3 as u128, 5u16);
        // D s_27_5: cmp-ne s_27_2 s_27_4
        let s_27_5: bool = ((s_27_2) != (s_27_4));
        // D s_27_6: write-var gs#31441 <= s_27_5
        fn_state.gs_31441 = s_27_5;
        // N s_27_7: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_28_0: const #1u : u8
        let s_28_0: bool = true;
        // D s_28_1: write-var gs#31440 <= s_28_0
        fn_state.gs_31440 = s_28_0;
        // N s_28_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_29_0: const #0u : u8
        let s_29_0: bool = false;
        // D s_29_1: write-var valid_name <= s_29_0
        fn_state.valid_name = s_29_0;
        // N s_29_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
