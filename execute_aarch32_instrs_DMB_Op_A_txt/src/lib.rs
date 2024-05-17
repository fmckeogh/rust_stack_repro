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
use u_get_HCR_Type_BSU::*;
use DataMemoryBarrier::*;
use HCR_read::*;
use EL2Enabled::*;
use common::*;
pub fn execute_aarch32_instrs_DMB_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    option_name: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        domain: u32,
        types: u32,
        gs_296530: bool,
        typesshadow_7147: u32,
        gs_296531: bool,
        gs_296528: bool,
        gs_296527: bool,
        option_name: u8,
    }
    let fn_state = FunctionState {
        option_name,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var option_name:u8
        let s_0_0: u8 = fn_state.option_name;
        // D s_0_1: cast zx s_0_0 -> bv
        let s_0_1: Bits = Bits::new(s_0_0 as u128, 4u16);
        // C s_0_2: const #1u : u8
        let s_0_2: u8 = 1;
        // C s_0_3: cast zx s_0_2 -> bv
        let s_0_3: Bits = Bits::new(s_0_2 as u128, 4u16);
        // D s_0_4: cmp-eq s_0_1 s_0_3
        let s_0_4: bool = ((s_0_1) == (s_0_3));
        // D s_0_5: not s_0_4
        let s_0_5: bool = !s_0_4;
        // N s_0_6: branch s_0_5 b27 b1
        if s_0_5 {
            return block_27(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #2u : u32
        let s_1_0: u32 = 2;
        // D s_1_1: write-var domain <= s_1_0
        fn_state.domain = s_1_0;
        // C s_1_2: const #0u : u32
        let s_1_2: u32 = 0;
        // D s_1_3: write-var types <= s_1_2
        fn_state.types = s_1_2;
        // N s_1_4: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var types:u32
        let s_2_0: u32 = fn_state.types;
        // D s_2_1: write-var typesshadow#7147 <= s_2_0
        fn_state.typesshadow_7147 = s_2_0;
        // C s_2_2: const #16975u : u32
        let s_2_2: u32 = 16975;
        // D s_2_3: read-reg s_2_2:u8
        let s_2_3: u8 = {
            let value = state.read_register::<u8>(s_2_2 as isize);
            tracer.read_register(s_2_2 as isize, value);
            value
        };
        // D s_2_4: cast zx s_2_3 -> bv
        let s_2_4: Bits = Bits::new(s_2_3 as u128, 2u16);
        // C s_2_5: const #448u : u32
        let s_2_5: u32 = 448;
        // D s_2_6: read-reg s_2_5:u8
        let s_2_6: u8 = {
            let value = state.read_register::<u8>(s_2_5 as isize);
            tracer.read_register(s_2_5 as isize, value);
            value
        };
        // D s_2_7: cast zx s_2_6 -> bv
        let s_2_7: Bits = Bits::new(s_2_6 as u128, 2u16);
        // D s_2_8: cmp-eq s_2_4 s_2_7
        let s_2_8: bool = ((s_2_4) == (s_2_7));
        // N s_2_9: branch s_2_8 b26 b3
        if s_2_8 {
            return block_26(state, tracer, fn_state);
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
        // C s_3_3: const #440u : u32
        let s_3_3: u32 = 440;
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
        // D s_3_7: write-var gs#296527 <= s_3_6
        fn_state.gs_296527 = s_3_6;
        // N s_3_8: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#296527:u8
        let s_4_0: bool = fn_state.gs_296527;
        // N s_4_1: branch s_4_0 b25 b5
        if s_4_0 {
            return block_25(state, tracer, fn_state);
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
        // D s_5_1: write-var gs#296528 <= s_5_0
        fn_state.gs_296528 = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var gs#296528:u8
        let s_6_0: bool = fn_state.gs_296528;
        // N s_6_1: branch s_6_0 b9 b7
        if s_6_0 {
            return block_9(state, tracer, fn_state);
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
        // D s_8_0: read-var domain:u32
        let s_8_0: u32 = fn_state.domain;
        // D s_8_1: read-var typesshadow#7147:u32
        let s_8_1: u32 = fn_state.typesshadow_7147;
        // D s_8_2: call DataMemoryBarrier(s_8_0, s_8_1)
        let s_8_2: () = DataMemoryBarrier(state, tracer, s_8_0, s_8_1);
        // N s_8_3: return
        return;
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #() : ()
        let s_9_0: () = ();
        // S s_9_1: call HCR_read(s_9_0)
        let s_9_1: ProductType700c18a878c5601b = HCR_read(state, tracer, s_9_0);
        // S s_9_2: call _get_HCR_Type_BSU(s_9_1)
        let s_9_2: u8 = u_get_HCR_Type_BSU(state, tracer, s_9_1);
        // S s_9_3: cast zx s_9_2 -> bv
        let s_9_3: Bits = Bits::new(s_9_2 as u128, 2u16);
        // C s_9_4: const #3u : u8
        let s_9_4: u8 = 3;
        // C s_9_5: cast zx s_9_4 -> bv
        let s_9_5: Bits = Bits::new(s_9_4 as u128, 2u16);
        // S s_9_6: cmp-eq s_9_3 s_9_5
        let s_9_6: bool = ((s_9_3) == (s_9_5));
        // N s_9_7: branch s_9_6 b24 b10
        if s_9_6 {
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
        // N s_10_0: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #() : ()
        let s_11_0: () = ();
        // S s_11_1: call HCR_read(s_11_0)
        let s_11_1: ProductType700c18a878c5601b = HCR_read(state, tracer, s_11_0);
        // S s_11_2: call _get_HCR_Type_BSU(s_11_1)
        let s_11_2: u8 = u_get_HCR_Type_BSU(state, tracer, s_11_1);
        // S s_11_3: cast zx s_11_2 -> bv
        let s_11_3: Bits = Bits::new(s_11_2 as u128, 2u16);
        // C s_11_4: const #2u : u8
        let s_11_4: u8 = 2;
        // C s_11_5: cast zx s_11_4 -> bv
        let s_11_5: Bits = Bits::new(s_11_4 as u128, 2u16);
        // S s_11_6: cmp-eq s_11_3 s_11_5
        let s_11_6: bool = ((s_11_3) == (s_11_5));
        // N s_11_7: branch s_11_6 b23 b12
        if s_11_6 {
            return block_23(state, tracer, fn_state);
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
        // D s_12_1: write-var gs#296530 <= s_12_0
        fn_state.gs_296530 = s_12_0;
        // N s_12_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var gs#296530:u8
        let s_13_0: bool = fn_state.gs_296530;
        // N s_13_1: branch s_13_0 b22 b14
        if s_13_0 {
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
        // N s_14_0: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #() : ()
        let s_15_0: () = ();
        // S s_15_1: call HCR_read(s_15_0)
        let s_15_1: ProductType700c18a878c5601b = HCR_read(state, tracer, s_15_0);
        // S s_15_2: call _get_HCR_Type_BSU(s_15_1)
        let s_15_2: u8 = u_get_HCR_Type_BSU(state, tracer, s_15_1);
        // S s_15_3: cast zx s_15_2 -> bv
        let s_15_3: Bits = Bits::new(s_15_2 as u128, 2u16);
        // C s_15_4: const #1u : u8
        let s_15_4: u8 = 1;
        // C s_15_5: cast zx s_15_4 -> bv
        let s_15_5: Bits = Bits::new(s_15_4 as u128, 2u16);
        // S s_15_6: cmp-eq s_15_3 s_15_5
        let s_15_6: bool = ((s_15_3) == (s_15_5));
        // N s_15_7: branch s_15_6 b21 b16
        if s_15_6 {
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
        // D s_16_1: write-var gs#296531 <= s_16_0
        fn_state.gs_296531 = s_16_0;
        // N s_16_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var gs#296531:u8
        let s_17_0: bool = fn_state.gs_296531;
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
        // N s_19_0: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_20_0: const #1u : u32
        let s_20_0: u32 = 1;
        // D s_20_1: write-var domain <= s_20_0
        fn_state.domain = s_20_0;
        // N s_20_2: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var domain:u32
        let s_21_0: u32 = fn_state.domain;
        // C s_21_1: const #0u : u32
        let s_21_1: u32 = 0;
        // D s_21_2: cmp-eq s_21_0 s_21_1
        let s_21_2: bool = ((s_21_0) == (s_21_1));
        // D s_21_3: write-var gs#296531 <= s_21_2
        fn_state.gs_296531 = s_21_2;
        // N s_21_4: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_22_0: const #2u : u32
        let s_22_0: u32 = 2;
        // D s_22_1: write-var domain <= s_22_0
        fn_state.domain = s_22_0;
        // N s_22_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_23_0: read-var domain:u32
        let s_23_0: u32 = fn_state.domain;
        // C s_23_1: const #3u : u32
        let s_23_1: u32 = 3;
        // D s_23_2: cmp-eq s_23_0 s_23_1
        let s_23_2: bool = ((s_23_0) == (s_23_1));
        // D s_23_3: write-var gs#296530 <= s_23_2
        fn_state.gs_296530 = s_23_2;
        // N s_23_4: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_24_0: const #3u : u32
        let s_24_0: u32 = 3;
        // D s_24_1: write-var domain <= s_24_0
        fn_state.domain = s_24_0;
        // N s_24_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_25_0: const #() : ()
        let s_25_0: () = ();
        // S s_25_1: call EL2Enabled(s_25_0)
        let s_25_1: bool = EL2Enabled(state, tracer, s_25_0);
        // D s_25_2: write-var gs#296528 <= s_25_1
        fn_state.gs_296528 = s_25_1;
        // N s_25_3: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_26_0: const #1u : u8
        let s_26_0: bool = true;
        // D s_26_1: write-var gs#296527 <= s_26_0
        fn_state.gs_296527 = s_26_0;
        // N s_26_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_27_0: read-var option_name:u8
        let s_27_0: u8 = fn_state.option_name;
        // D s_27_1: cast zx s_27_0 -> bv
        let s_27_1: Bits = Bits::new(s_27_0 as u128, 4u16);
        // C s_27_2: const #2u : u8
        let s_27_2: u8 = 2;
        // C s_27_3: cast zx s_27_2 -> bv
        let s_27_3: Bits = Bits::new(s_27_2 as u128, 4u16);
        // D s_27_4: cmp-eq s_27_1 s_27_3
        let s_27_4: bool = ((s_27_1) == (s_27_3));
        // D s_27_5: not s_27_4
        let s_27_5: bool = !s_27_4;
        // N s_27_6: branch s_27_5 b29 b28
        if s_27_5 {
            return block_29(state, tracer, fn_state);
        } else {
            return block_28(state, tracer, fn_state);
        };
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_28_0: const #2u : u32
        let s_28_0: u32 = 2;
        // D s_28_1: write-var domain <= s_28_0
        fn_state.domain = s_28_0;
        // C s_28_2: const #1u : u32
        let s_28_2: u32 = 1;
        // D s_28_3: write-var types <= s_28_2
        fn_state.types = s_28_2;
        // N s_28_4: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_29_0: read-var option_name:u8
        let s_29_0: u8 = fn_state.option_name;
        // D s_29_1: cast zx s_29_0 -> bv
        let s_29_1: Bits = Bits::new(s_29_0 as u128, 4u16);
        // C s_29_2: const #3u : u8
        let s_29_2: u8 = 3;
        // C s_29_3: cast zx s_29_2 -> bv
        let s_29_3: Bits = Bits::new(s_29_2 as u128, 4u16);
        // D s_29_4: cmp-eq s_29_1 s_29_3
        let s_29_4: bool = ((s_29_1) == (s_29_3));
        // D s_29_5: not s_29_4
        let s_29_5: bool = !s_29_4;
        // N s_29_6: branch s_29_5 b31 b30
        if s_29_5 {
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
        // C s_30_0: const #2u : u32
        let s_30_0: u32 = 2;
        // D s_30_1: write-var domain <= s_30_0
        fn_state.domain = s_30_0;
        // C s_30_2: const #2u : u32
        let s_30_2: u32 = 2;
        // D s_30_3: write-var types <= s_30_2
        fn_state.types = s_30_2;
        // N s_30_4: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_31_0: read-var option_name:u8
        let s_31_0: u8 = fn_state.option_name;
        // D s_31_1: cast zx s_31_0 -> bv
        let s_31_1: Bits = Bits::new(s_31_0 as u128, 4u16);
        // C s_31_2: const #5u : u8
        let s_31_2: u8 = 5;
        // C s_31_3: cast zx s_31_2 -> bv
        let s_31_3: Bits = Bits::new(s_31_2 as u128, 4u16);
        // D s_31_4: cmp-eq s_31_1 s_31_3
        let s_31_4: bool = ((s_31_1) == (s_31_3));
        // D s_31_5: not s_31_4
        let s_31_5: bool = !s_31_4;
        // N s_31_6: branch s_31_5 b33 b32
        if s_31_5 {
            return block_33(state, tracer, fn_state);
        } else {
            return block_32(state, tracer, fn_state);
        };
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_32_0: const #0u : u32
        let s_32_0: u32 = 0;
        // D s_32_1: write-var domain <= s_32_0
        fn_state.domain = s_32_0;
        // C s_32_2: const #0u : u32
        let s_32_2: u32 = 0;
        // D s_32_3: write-var types <= s_32_2
        fn_state.types = s_32_2;
        // N s_32_4: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_33_0: read-var option_name:u8
        let s_33_0: u8 = fn_state.option_name;
        // D s_33_1: cast zx s_33_0 -> bv
        let s_33_1: Bits = Bits::new(s_33_0 as u128, 4u16);
        // C s_33_2: const #6u : u8
        let s_33_2: u8 = 6;
        // C s_33_3: cast zx s_33_2 -> bv
        let s_33_3: Bits = Bits::new(s_33_2 as u128, 4u16);
        // D s_33_4: cmp-eq s_33_1 s_33_3
        let s_33_4: bool = ((s_33_1) == (s_33_3));
        // D s_33_5: not s_33_4
        let s_33_5: bool = !s_33_4;
        // N s_33_6: branch s_33_5 b35 b34
        if s_33_5 {
            return block_35(state, tracer, fn_state);
        } else {
            return block_34(state, tracer, fn_state);
        };
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_34_0: const #0u : u32
        let s_34_0: u32 = 0;
        // D s_34_1: write-var domain <= s_34_0
        fn_state.domain = s_34_0;
        // C s_34_2: const #1u : u32
        let s_34_2: u32 = 1;
        // D s_34_3: write-var types <= s_34_2
        fn_state.types = s_34_2;
        // N s_34_4: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_35_0: read-var option_name:u8
        let s_35_0: u8 = fn_state.option_name;
        // D s_35_1: cast zx s_35_0 -> bv
        let s_35_1: Bits = Bits::new(s_35_0 as u128, 4u16);
        // C s_35_2: const #7u : u8
        let s_35_2: u8 = 7;
        // C s_35_3: cast zx s_35_2 -> bv
        let s_35_3: Bits = Bits::new(s_35_2 as u128, 4u16);
        // D s_35_4: cmp-eq s_35_1 s_35_3
        let s_35_4: bool = ((s_35_1) == (s_35_3));
        // D s_35_5: not s_35_4
        let s_35_5: bool = !s_35_4;
        // N s_35_6: branch s_35_5 b37 b36
        if s_35_5 {
            return block_37(state, tracer, fn_state);
        } else {
            return block_36(state, tracer, fn_state);
        };
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_36_0: const #0u : u32
        let s_36_0: u32 = 0;
        // D s_36_1: write-var domain <= s_36_0
        fn_state.domain = s_36_0;
        // C s_36_2: const #2u : u32
        let s_36_2: u32 = 2;
        // D s_36_3: write-var types <= s_36_2
        fn_state.types = s_36_2;
        // N s_36_4: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_37_0: read-var option_name:u8
        let s_37_0: u8 = fn_state.option_name;
        // D s_37_1: cast zx s_37_0 -> bv
        let s_37_1: Bits = Bits::new(s_37_0 as u128, 4u16);
        // C s_37_2: const #9u : u8
        let s_37_2: u8 = 9;
        // C s_37_3: cast zx s_37_2 -> bv
        let s_37_3: Bits = Bits::new(s_37_2 as u128, 4u16);
        // D s_37_4: cmp-eq s_37_1 s_37_3
        let s_37_4: bool = ((s_37_1) == (s_37_3));
        // D s_37_5: not s_37_4
        let s_37_5: bool = !s_37_4;
        // N s_37_6: branch s_37_5 b39 b38
        if s_37_5 {
            return block_39(state, tracer, fn_state);
        } else {
            return block_38(state, tracer, fn_state);
        };
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_38_0: const #1u : u32
        let s_38_0: u32 = 1;
        // D s_38_1: write-var domain <= s_38_0
        fn_state.domain = s_38_0;
        // C s_38_2: const #0u : u32
        let s_38_2: u32 = 0;
        // D s_38_3: write-var types <= s_38_2
        fn_state.types = s_38_2;
        // N s_38_4: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_39_0: read-var option_name:u8
        let s_39_0: u8 = fn_state.option_name;
        // D s_39_1: cast zx s_39_0 -> bv
        let s_39_1: Bits = Bits::new(s_39_0 as u128, 4u16);
        // C s_39_2: const #10u : u8
        let s_39_2: u8 = 10;
        // C s_39_3: cast zx s_39_2 -> bv
        let s_39_3: Bits = Bits::new(s_39_2 as u128, 4u16);
        // D s_39_4: cmp-eq s_39_1 s_39_3
        let s_39_4: bool = ((s_39_1) == (s_39_3));
        // D s_39_5: not s_39_4
        let s_39_5: bool = !s_39_4;
        // N s_39_6: branch s_39_5 b41 b40
        if s_39_5 {
            return block_41(state, tracer, fn_state);
        } else {
            return block_40(state, tracer, fn_state);
        };
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_40_0: const #1u : u32
        let s_40_0: u32 = 1;
        // D s_40_1: write-var domain <= s_40_0
        fn_state.domain = s_40_0;
        // C s_40_2: const #1u : u32
        let s_40_2: u32 = 1;
        // D s_40_3: write-var types <= s_40_2
        fn_state.types = s_40_2;
        // N s_40_4: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_41_0: read-var option_name:u8
        let s_41_0: u8 = fn_state.option_name;
        // D s_41_1: cast zx s_41_0 -> bv
        let s_41_1: Bits = Bits::new(s_41_0 as u128, 4u16);
        // C s_41_2: const #11u : u8
        let s_41_2: u8 = 11;
        // C s_41_3: cast zx s_41_2 -> bv
        let s_41_3: Bits = Bits::new(s_41_2 as u128, 4u16);
        // D s_41_4: cmp-eq s_41_1 s_41_3
        let s_41_4: bool = ((s_41_1) == (s_41_3));
        // D s_41_5: not s_41_4
        let s_41_5: bool = !s_41_4;
        // N s_41_6: branch s_41_5 b43 b42
        if s_41_5 {
            return block_43(state, tracer, fn_state);
        } else {
            return block_42(state, tracer, fn_state);
        };
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_42_0: const #1u : u32
        let s_42_0: u32 = 1;
        // D s_42_1: write-var domain <= s_42_0
        fn_state.domain = s_42_0;
        // C s_42_2: const #2u : u32
        let s_42_2: u32 = 2;
        // D s_42_3: write-var types <= s_42_2
        fn_state.types = s_42_2;
        // N s_42_4: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_43_0: read-var option_name:u8
        let s_43_0: u8 = fn_state.option_name;
        // D s_43_1: cast zx s_43_0 -> bv
        let s_43_1: Bits = Bits::new(s_43_0 as u128, 4u16);
        // C s_43_2: const #13u : u8
        let s_43_2: u8 = 13;
        // C s_43_3: cast zx s_43_2 -> bv
        let s_43_3: Bits = Bits::new(s_43_2 as u128, 4u16);
        // D s_43_4: cmp-eq s_43_1 s_43_3
        let s_43_4: bool = ((s_43_1) == (s_43_3));
        // D s_43_5: not s_43_4
        let s_43_5: bool = !s_43_4;
        // N s_43_6: branch s_43_5 b45 b44
        if s_43_5 {
            return block_45(state, tracer, fn_state);
        } else {
            return block_44(state, tracer, fn_state);
        };
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_44_0: const #3u : u32
        let s_44_0: u32 = 3;
        // D s_44_1: write-var domain <= s_44_0
        fn_state.domain = s_44_0;
        // C s_44_2: const #0u : u32
        let s_44_2: u32 = 0;
        // D s_44_3: write-var types <= s_44_2
        fn_state.types = s_44_2;
        // N s_44_4: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_45_0: read-var option_name:u8
        let s_45_0: u8 = fn_state.option_name;
        // D s_45_1: cast zx s_45_0 -> bv
        let s_45_1: Bits = Bits::new(s_45_0 as u128, 4u16);
        // C s_45_2: const #14u : u8
        let s_45_2: u8 = 14;
        // C s_45_3: cast zx s_45_2 -> bv
        let s_45_3: Bits = Bits::new(s_45_2 as u128, 4u16);
        // D s_45_4: cmp-eq s_45_1 s_45_3
        let s_45_4: bool = ((s_45_1) == (s_45_3));
        // D s_45_5: not s_45_4
        let s_45_5: bool = !s_45_4;
        // N s_45_6: branch s_45_5 b47 b46
        if s_45_5 {
            return block_47(state, tracer, fn_state);
        } else {
            return block_46(state, tracer, fn_state);
        };
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_46_0: const #3u : u32
        let s_46_0: u32 = 3;
        // D s_46_1: write-var domain <= s_46_0
        fn_state.domain = s_46_0;
        // C s_46_2: const #1u : u32
        let s_46_2: u32 = 1;
        // D s_46_3: write-var types <= s_46_2
        fn_state.types = s_46_2;
        // N s_46_4: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_47_0: const #3u : u32
        let s_47_0: u32 = 3;
        // D s_47_1: write-var domain <= s_47_0
        fn_state.domain = s_47_0;
        // C s_47_2: const #2u : u32
        let s_47_2: u32 = 2;
        // D s_47_3: write-var types <= s_47_2
        fn_state.types = s_47_2;
        // N s_47_4: jump b2
        return block_2(state, tracer, fn_state);
    }
}
