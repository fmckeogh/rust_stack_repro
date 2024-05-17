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
use u_get_CPACR_EL1_Type_FPEN::*;
use IsInHost::*;
use AArch64_CheckFPAdvSIMDTrap::*;
use AArch64_AdvSIMDFPAccessTrap::*;
use common::*;
pub fn AArch64_CheckFPEnabled<T: Tracer>(
    state: &mut State,
    tracer: &T,
    gs_25076: (),
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_25078: bool,
        disabled: bool,
        gs_25077: bool,
        ga_19585: u8,
        gs_25076: (),
    }
    let fn_state = FunctionState {
        gs_25076,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #16975u : u32
        let s_0_0: u32 = 16975;
        // D s_0_1: read-reg s_0_0:u8
        let s_0_1: u8 = {
            let value = state.read_register::<u8>(s_0_0 as isize);
            tracer.read_register(s_0_0 as isize, value);
            value
        };
        // D s_0_2: cast zx s_0_1 -> bv
        let s_0_2: Bits = Bits::new(s_0_1 as u128, 2u16);
        // C s_0_3: const #448u : u32
        let s_0_3: u32 = 448;
        // D s_0_4: read-reg s_0_3:u8
        let s_0_4: u8 = {
            let value = state.read_register::<u8>(s_0_3 as isize);
            tracer.read_register(s_0_3 as isize, value);
            value
        };
        // D s_0_5: cast zx s_0_4 -> bv
        let s_0_5: Bits = Bits::new(s_0_4 as u128, 2u16);
        // D s_0_6: cmp-eq s_0_2 s_0_5
        let s_0_6: bool = ((s_0_2) == (s_0_5));
        // N s_0_7: branch s_0_6 b17 b1
        if s_0_6 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #16975u : u32
        let s_1_0: u32 = 16975;
        // D s_1_1: read-reg s_1_0:u8
        let s_1_1: u8 = {
            let value = state.read_register::<u8>(s_1_0 as isize);
            tracer.read_register(s_1_0 as isize, value);
            value
        };
        // D s_1_2: cast zx s_1_1 -> bv
        let s_1_2: Bits = Bits::new(s_1_1 as u128, 2u16);
        // C s_1_3: const #440u : u32
        let s_1_3: u32 = 440;
        // D s_1_4: read-reg s_1_3:u8
        let s_1_4: u8 = {
            let value = state.read_register::<u8>(s_1_3 as isize);
            tracer.read_register(s_1_3 as isize, value);
            value
        };
        // D s_1_5: cast zx s_1_4 -> bv
        let s_1_5: Bits = Bits::new(s_1_4 as u128, 2u16);
        // D s_1_6: cmp-eq s_1_2 s_1_5
        let s_1_6: bool = ((s_1_2) == (s_1_5));
        // D s_1_7: write-var gs#25077 <= s_1_6
        fn_state.gs_25077 = s_1_6;
        // N s_1_8: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#25077:u8
        let s_2_0: bool = fn_state.gs_25077;
        // N s_2_1: branch s_2_0 b16 b3
        if s_2_0 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #0u : u8
        let s_3_0: bool = false;
        // D s_3_1: write-var gs#25078 <= s_3_0
        fn_state.gs_25078 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#25078:u8
        let s_4_0: bool = fn_state.gs_25078;
        // N s_4_1: branch s_4_0 b7 b5
        if s_4_0 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_5_0: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #() : ()
        let s_6_0: () = ();
        // S s_6_1: call AArch64_CheckFPAdvSIMDTrap(s_6_0)
        let s_6_1: () = AArch64_CheckFPAdvSIMDTrap(state, tracer, s_6_0);
        // N s_6_2: return
        return;
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #12088u : u32
        let s_7_0: u32 = 12088;
        // D s_7_1: read-reg s_7_0:struct
        let s_7_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_7_0 as isize);
            tracer.read_register(s_7_0 as isize, value);
            value
        };
        // D s_7_2: call _get_CPACR_EL1_Type_FPEN(s_7_1)
        let s_7_2: u8 = u_get_CPACR_EL1_Type_FPEN(state, tracer, s_7_1);
        // D s_7_3: write-var ga#19585 <= s_7_2
        fn_state.ga_19585 = s_7_2;
        // D s_7_4: read-var ga#19585:u8
        let s_7_4: u8 = fn_state.ga_19585;
        // C s_7_5: const #0s : i
        let s_7_5: i128 = 0;
        // D s_7_6: cast zx s_7_4 -> bv
        let s_7_6: Bits = Bits::new(s_7_4 as u128, 2u16);
        // C s_7_7: const #1s : i64
        let s_7_7: i64 = 1;
        // C s_7_8: cast zx s_7_7 -> i
        let s_7_8: i128 = (i128::try_from(s_7_7).unwrap());
        // C s_7_9: const #0s : i
        let s_7_9: i128 = 0;
        // C s_7_10: add s_7_9 s_7_8
        let s_7_10: i128 = (s_7_9 + s_7_8);
        // D s_7_11: bit-extract s_7_6 s_7_5 s_7_10
        let s_7_11: Bits = (Bits::new(
            ((s_7_6) >> (s_7_5)).value(),
            u16::try_from(s_7_10).unwrap(),
        ));
        // D s_7_12: cast reint s_7_11 -> u8
        let s_7_12: bool = ((s_7_11.value()) != 0);
        // D s_7_13: cast zx s_7_12 -> bv
        let s_7_13: Bits = Bits::new(s_7_12 as u128, 1u16);
        // C s_7_14: const #0u : u8
        let s_7_14: bool = false;
        // C s_7_15: cast zx s_7_14 -> bv
        let s_7_15: Bits = Bits::new(s_7_14 as u128, 1u16);
        // D s_7_16: cmp-eq s_7_13 s_7_15
        let s_7_16: bool = ((s_7_13) == (s_7_15));
        // D s_7_17: not s_7_16
        let s_7_17: bool = !s_7_16;
        // N s_7_18: branch s_7_17 b13 b8
        if s_7_17 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #1u : u8
        let s_8_0: bool = true;
        // D s_8_1: write-var disabled <= s_8_0
        fn_state.disabled = s_8_0;
        // N s_8_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var disabled:u8
        let s_9_0: bool = fn_state.disabled;
        // N s_9_1: branch s_9_0 b12 b10
        if s_9_0 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_10_0: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_11_0: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #440u : u32
        let s_12_0: u32 = 440;
        // D s_12_1: read-reg s_12_0:u8
        let s_12_1: u8 = {
            let value = state.read_register::<u8>(s_12_0 as isize);
            tracer.read_register(s_12_0 as isize, value);
            value
        };
        // D s_12_2: call AArch64_AdvSIMDFPAccessTrap(s_12_1)
        let s_12_2: () = AArch64_AdvSIMDFPAccessTrap(state, tracer, s_12_1);
        // N s_12_3: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var ga#19585:u8
        let s_13_0: u8 = fn_state.ga_19585;
        // D s_13_1: cast zx s_13_0 -> bv
        let s_13_1: Bits = Bits::new(s_13_0 as u128, 2u16);
        // C s_13_2: const #1u : u8
        let s_13_2: u8 = 1;
        // C s_13_3: cast zx s_13_2 -> bv
        let s_13_3: Bits = Bits::new(s_13_2 as u128, 2u16);
        // D s_13_4: cmp-eq s_13_1 s_13_3
        let s_13_4: bool = ((s_13_1) == (s_13_3));
        // D s_13_5: not s_13_4
        let s_13_5: bool = !s_13_4;
        // N s_13_6: branch s_13_5 b15 b14
        if s_13_5 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #16975u : u32
        let s_14_0: u32 = 16975;
        // D s_14_1: read-reg s_14_0:u8
        let s_14_1: u8 = {
            let value = state.read_register::<u8>(s_14_0 as isize);
            tracer.read_register(s_14_0 as isize, value);
            value
        };
        // D s_14_2: cast zx s_14_1 -> bv
        let s_14_2: Bits = Bits::new(s_14_1 as u128, 2u16);
        // C s_14_3: const #448u : u32
        let s_14_3: u32 = 448;
        // D s_14_4: read-reg s_14_3:u8
        let s_14_4: u8 = {
            let value = state.read_register::<u8>(s_14_3 as isize);
            tracer.read_register(s_14_3 as isize, value);
            value
        };
        // D s_14_5: cast zx s_14_4 -> bv
        let s_14_5: Bits = Bits::new(s_14_4 as u128, 2u16);
        // D s_14_6: cmp-eq s_14_2 s_14_5
        let s_14_6: bool = ((s_14_2) == (s_14_5));
        // D s_14_7: write-var disabled <= s_14_6
        fn_state.disabled = s_14_6;
        // N s_14_8: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #0u : u8
        let s_15_0: bool = false;
        // D s_15_1: write-var disabled <= s_15_0
        fn_state.disabled = s_15_0;
        // N s_15_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #() : ()
        let s_16_0: () = ();
        // S s_16_1: call IsInHost(s_16_0)
        let s_16_1: bool = IsInHost(state, tracer, s_16_0);
        // S s_16_2: not s_16_1
        let s_16_2: bool = !s_16_1;
        // D s_16_3: write-var gs#25078 <= s_16_2
        fn_state.gs_25078 = s_16_2;
        // N s_16_4: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_17_0: const #1u : u8
        let s_17_0: bool = true;
        // D s_17_1: write-var gs#25077 <= s_17_0
        fn_state.gs_25077 = s_17_0;
        // N s_17_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
