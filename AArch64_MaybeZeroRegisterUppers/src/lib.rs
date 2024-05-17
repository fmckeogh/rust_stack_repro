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
use neq_int::*;
use ConstrainUnpredictableBool::*;
use ELUsingAArch32::*;
use UsingAArch32::*;
use set_R_bits::*;
use Zeros::*;
use EL2Enabled::*;
use common::*;
pub fn AArch64_MaybeZeroRegisterUppers<T: Tracer>(
    state: &mut State,
    tracer: &T,
    gs_5682: (),
) -> () {
    #[derive(Default)]
    struct FunctionState {
        last: i64,
        gs_5690: bool,
        gs_5706: i64,
        gs_5710: bool,
        n: i64,
        gs_5711: bool,
        gs_5689: bool,
        gs_5686: bool,
        first: i64,
        gs_5688: bool,
        include_R15_name: bool,
        gs_5682: (),
    }
    let fn_state = FunctionState {
        gs_5682,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #() : ()
        let s_0_0: () = ();
        // S s_0_1: call UsingAArch32(s_0_0)
        let s_0_1: bool = UsingAArch32(state, tracer, s_0_0);
        // N s_0_2: assert s_0_1
        let s_0_2: () = assert!(s_0_1);
        // C s_0_3: const #0s : i64
        let s_0_3: i64 = 0;
        // D s_0_4: write-var first <= s_0_3
        fn_state.first = s_0_3;
        // C s_0_5: const #14s : i64
        let s_0_5: i64 = 14;
        // D s_0_6: write-var last <= s_0_5
        fn_state.last = s_0_5;
        // C s_0_7: const #16975u : u32
        let s_0_7: u32 = 16975;
        // D s_0_8: read-reg s_0_7:u8
        let s_0_8: u8 = {
            let value = state.read_register::<u8>(s_0_7 as isize);
            tracer.read_register(s_0_7 as isize, value);
            value
        };
        // D s_0_9: cast zx s_0_8 -> bv
        let s_0_9: Bits = Bits::new(s_0_8 as u128, 2u16);
        // C s_0_10: const #448u : u32
        let s_0_10: u32 = 448;
        // D s_0_11: read-reg s_0_10:u8
        let s_0_11: u8 = {
            let value = state.read_register::<u8>(s_0_10 as isize);
            tracer.read_register(s_0_10 as isize, value);
            value
        };
        // D s_0_12: cast zx s_0_11 -> bv
        let s_0_12: Bits = Bits::new(s_0_11 as u128, 2u16);
        // D s_0_13: cmp-eq s_0_9 s_0_12
        let s_0_13: bool = ((s_0_9) == (s_0_12));
        // N s_0_14: branch s_0_13 b29 b1
        if s_0_13 {
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
        // C s_1_0: const #0u : u8
        let s_1_0: bool = false;
        // D s_1_1: write-var gs#5686 <= s_1_0
        fn_state.gs_5686 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#5686:u8
        let s_2_0: bool = fn_state.gs_5686;
        // N s_2_1: branch s_2_0 b28 b3
        if s_2_0 {
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
        // C s_3_0: const #16975u : u32
        let s_3_0: u32 = 16975;
        // D s_3_1: read-reg s_3_0:u8
        let s_3_1: u8 = {
            let value = state.read_register::<u8>(s_3_0 as isize);
            tracer.read_register(s_3_0 as isize, value);
            value
        };
        // D s_3_2: cast zx s_3_1 -> bv
        let s_3_2: Bits = Bits::new(s_3_1 as u128, 2u16);
        // C s_3_3: const #448u : u32
        let s_3_3: u32 = 448;
        // D s_3_4: read-reg s_3_3:u8
        let s_3_4: u8 = {
            let value = state.read_register::<u8>(s_3_3 as isize);
            tracer.read_register(s_3_3 as isize, value);
            value
        };
        // D s_3_5: cast zx s_3_4 -> bv
        let s_3_5: Bits = Bits::new(s_3_4 as u128, 2u16);
        // D s_3_6: cmp-eq s_3_2 s_3_5
        let s_3_6: bool = ((s_3_2) == (s_3_5));
        // N s_3_7: branch s_3_6 b27 b4
        if s_3_6 {
            return block_27(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #16975u : u32
        let s_4_0: u32 = 16975;
        // D s_4_1: read-reg s_4_0:u8
        let s_4_1: u8 = {
            let value = state.read_register::<u8>(s_4_0 as isize);
            tracer.read_register(s_4_0 as isize, value);
            value
        };
        // D s_4_2: cast zx s_4_1 -> bv
        let s_4_2: Bits = Bits::new(s_4_1 as u128, 2u16);
        // C s_4_3: const #440u : u32
        let s_4_3: u32 = 440;
        // D s_4_4: read-reg s_4_3:u8
        let s_4_4: u8 = {
            let value = state.read_register::<u8>(s_4_3 as isize);
            tracer.read_register(s_4_3 as isize, value);
            value
        };
        // D s_4_5: cast zx s_4_4 -> bv
        let s_4_5: Bits = Bits::new(s_4_4 as u128, 2u16);
        // D s_4_6: cmp-eq s_4_2 s_4_5
        let s_4_6: bool = ((s_4_2) == (s_4_5));
        // D s_4_7: write-var gs#5688 <= s_4_6
        fn_state.gs_5688 = s_4_6;
        // N s_4_8: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var gs#5688:u8
        let s_5_0: bool = fn_state.gs_5688;
        // N s_5_1: branch s_5_0 b26 b6
        if s_5_0 {
            return block_26(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #0u : u8
        let s_6_0: bool = false;
        // D s_6_1: write-var gs#5689 <= s_6_0
        fn_state.gs_5689 = s_6_0;
        // N s_6_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var gs#5689:u8
        let s_7_0: bool = fn_state.gs_5689;
        // N s_7_1: branch s_7_0 b25 b8
        if s_7_0 {
            return block_25(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #0u : u8
        let s_8_0: bool = false;
        // D s_8_1: write-var gs#5690 <= s_8_0
        fn_state.gs_5690 = s_8_0;
        // N s_8_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var gs#5690:u8
        let s_9_0: bool = fn_state.gs_5690;
        // N s_9_1: branch s_9_0 b24 b10
        if s_9_0 {
            return block_24(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #0s : i64
        let s_10_0: i64 = 0;
        // D s_10_1: write-var first <= s_10_0
        fn_state.first = s_10_0;
        // C s_10_2: const #30s : i64
        let s_10_2: i64 = 30;
        // D s_10_3: write-var last <= s_10_2
        fn_state.last = s_10_2;
        // C s_10_4: const #1u : u8
        let s_10_4: bool = true;
        // D s_10_5: write-var include_R15_name <= s_10_4
        fn_state.include_R15_name = s_10_4;
        // N s_10_6: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var last:i64
        let s_11_0: i64 = fn_state.last;
        // D s_11_1: read-var first:i64
        let s_11_1: i64 = fn_state.first;
        // D s_11_2: write-var gs#5706 <= s_11_0
        fn_state.gs_5706 = s_11_0;
        // D s_11_3: write-var n <= s_11_1
        fn_state.n = s_11_1;
        // N s_11_4: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var n:i64
        let s_12_0: i64 = fn_state.n;
        // D s_12_1: read-var gs#5706:i64
        let s_12_1: i64 = fn_state.gs_5706;
        // D s_12_2: cmp-gt s_12_0 s_12_1
        let s_12_2: bool = ((s_12_0) > (s_12_1));
        // N s_12_3: branch s_12_2 b23 b13
        if s_12_2 {
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
        // C s_13_0: const #15s : i
        let s_13_0: i128 = 15;
        // D s_13_1: read-var n:i64
        let s_13_1: i64 = fn_state.n;
        // D s_13_2: cast zx s_13_1 -> i
        let s_13_2: i128 = (i128::try_from(s_13_1).unwrap());
        // D s_13_3: call neq_int(s_13_2, s_13_0)
        let s_13_3: bool = neq_int(state, tracer, s_13_2, s_13_0);
        // N s_13_4: branch s_13_3 b22 b14
        if s_13_3 {
            return block_22(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var include_R15_name:u8
        let s_14_0: bool = fn_state.include_R15_name;
        // D s_14_1: write-var gs#5710 <= s_14_0
        fn_state.gs_5710 = s_14_0;
        // N s_14_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var gs#5710:u8
        let s_15_0: bool = fn_state.gs_5710;
        // N s_15_1: branch s_15_0 b21 b16
        if s_15_0 {
            return block_21(state, tracer, fn_state);
        } else {
            return block_16(state, tracer, fn_state);
        };
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #0u : u8
        let s_16_0: bool = false;
        // D s_16_1: write-var gs#5711 <= s_16_0
        fn_state.gs_5711 = s_16_0;
        // N s_16_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var gs#5711:u8
        let s_17_0: bool = fn_state.gs_5711;
        // N s_17_1: branch s_17_0 b20 b18
        if s_17_0 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_18(state, tracer, fn_state);
        };
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_18_0: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var n:i64
        let s_19_0: i64 = fn_state.n;
        // C s_19_1: const #1s : i64
        let s_19_1: i64 = 1;
        // D s_19_2: add s_19_0 s_19_1
        let s_19_2: i64 = (s_19_0 + s_19_1);
        // D s_19_3: write-var n <= s_19_2
        fn_state.n = s_19_2;
        // N s_19_4: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_20_0: const #32s : i
        let s_20_0: i128 = 32;
        // S s_20_1: call Zeros(s_20_0)
        let s_20_1: Bits = Zeros(state, tracer, s_20_0);
        // S s_20_2: cast reint s_20_1 -> u32
        let s_20_2: u32 = (s_20_1.value() as u32);
        // C s_20_3: const #63s : i64
        let s_20_3: i64 = 63;
        // C s_20_4: const #32s : i64
        let s_20_4: i64 = 32;
        // S s_20_5: cast zx s_20_2 -> bv
        let s_20_5: Bits = Bits::new(s_20_2 as u128, 32u16);
        // D s_20_6: read-var n:i64
        let s_20_6: i64 = fn_state.n;
        // D s_20_7: call set_R_bits(s_20_6, s_20_3, s_20_4, s_20_5)
        let s_20_7: () = set_R_bits(state, tracer, s_20_6, s_20_3, s_20_4, s_20_5);
        // N s_20_8: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_21_0: const #45u : u32
        let s_21_0: u32 = 45;
        // S s_21_1: call ConstrainUnpredictableBool(s_21_0)
        let s_21_1: bool = ConstrainUnpredictableBool(state, tracer, s_21_0);
        // D s_21_2: write-var gs#5711 <= s_21_1
        fn_state.gs_5711 = s_21_1;
        // N s_21_3: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_22_0: const #1u : u8
        let s_22_0: bool = true;
        // D s_22_1: write-var gs#5710 <= s_22_0
        fn_state.gs_5710 = s_22_0;
        // N s_22_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_23_0: return
        return;
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_24_0: const #0s : i64
        let s_24_0: i64 = 0;
        // D s_24_1: write-var first <= s_24_0
        fn_state.first = s_24_0;
        // C s_24_2: const #30s : i64
        let s_24_2: i64 = 30;
        // D s_24_3: write-var last <= s_24_2
        fn_state.last = s_24_2;
        // C s_24_4: const #0u : u8
        let s_24_4: bool = false;
        // D s_24_5: write-var include_R15_name <= s_24_4
        fn_state.include_R15_name = s_24_4;
        // N s_24_6: jump b11
        return block_11(state, tracer, fn_state);
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
        // D s_25_2: call ELUsingAArch32(s_25_1)
        let s_25_2: bool = ELUsingAArch32(state, tracer, s_25_1);
        // D s_25_3: not s_25_2
        let s_25_3: bool = !s_25_2;
        // D s_25_4: write-var gs#5690 <= s_25_3
        fn_state.gs_5690 = s_25_3;
        // N s_25_5: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_26_0: const #() : ()
        let s_26_0: () = ();
        // S s_26_1: call EL2Enabled(s_26_0)
        let s_26_1: bool = EL2Enabled(state, tracer, s_26_0);
        // D s_26_2: write-var gs#5689 <= s_26_1
        fn_state.gs_5689 = s_26_1;
        // N s_26_3: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_27_0: const #1u : u8
        let s_27_0: bool = true;
        // D s_27_1: write-var gs#5688 <= s_27_0
        fn_state.gs_5688 = s_27_0;
        // N s_27_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_28_0: const #0s : i64
        let s_28_0: i64 = 0;
        // D s_28_1: write-var first <= s_28_0
        fn_state.first = s_28_0;
        // C s_28_2: const #14s : i64
        let s_28_2: i64 = 14;
        // D s_28_3: write-var last <= s_28_2
        fn_state.last = s_28_2;
        // C s_28_4: const #0u : u8
        let s_28_4: bool = false;
        // D s_28_5: write-var include_R15_name <= s_28_4
        fn_state.include_R15_name = s_28_4;
        // N s_28_6: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_29_0: const #440u : u32
        let s_29_0: u32 = 440;
        // D s_29_1: read-reg s_29_0:u8
        let s_29_1: u8 = {
            let value = state.read_register::<u8>(s_29_0 as isize);
            tracer.read_register(s_29_0 as isize, value);
            value
        };
        // D s_29_2: call ELUsingAArch32(s_29_1)
        let s_29_2: bool = ELUsingAArch32(state, tracer, s_29_1);
        // D s_29_3: not s_29_2
        let s_29_3: bool = !s_29_2;
        // D s_29_4: write-var gs#5686 <= s_29_3
        fn_state.gs_5686 = s_29_3;
        // N s_29_5: jump b2
        return block_2(state, tracer, fn_state);
    }
}
