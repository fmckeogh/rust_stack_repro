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
use IsHCRXEL2Enabled::*;
use DataSynchronizationBarrier::*;
use HCR_read::*;
use ELUsingAArch32::*;
use HaveFeatXS::*;
use EL2Enabled::*;
use u_get_HCRX_EL2_Type_FnXS::*;
use common::*;
pub fn execute_aarch32_instrs_DSB_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    option_name: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        domain: u32,
        types: u32,
        nXS: bool,
        b__11: u8,
        gs_296580: bool,
        gs_296541: bool,
        gs_296570: bool,
        gs_296539: bool,
        gs_296540: bool,
        gs_296581: bool,
        gs_296565: bool,
        gs_296577: bool,
        typesshadow_7148: u32,
        gs_296538: bool,
        gs_296578: bool,
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
        // C s_0_0: const #() : ()
        let s_0_0: () = ();
        // S s_0_1: call HaveFeatXS(s_0_0)
        let s_0_1: bool = HaveFeatXS(state, tracer, s_0_0);
        // N s_0_2: branch s_0_1 b56 b1
        if s_0_1 {
            return block_56(state, tracer, fn_state);
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
        // D s_1_1: write-var nXS <= s_1_0
        fn_state.nXS = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var option_name:u8
        let s_2_0: u8 = fn_state.option_name;
        // D s_2_1: cast zx s_2_0 -> bv
        let s_2_1: Bits = Bits::new(s_2_0 as u128, 4u16);
        // C s_2_2: const #1u : u8
        let s_2_2: u8 = 1;
        // C s_2_3: cast zx s_2_2 -> bv
        let s_2_3: Bits = Bits::new(s_2_2 as u128, 4u16);
        // D s_2_4: cmp-eq s_2_1 s_2_3
        let s_2_4: bool = ((s_2_1) == (s_2_3));
        // D s_2_5: not s_2_4
        let s_2_5: bool = !s_2_4;
        // N s_2_6: branch s_2_5 b29 b3
        if s_2_5 {
            return block_29(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #2u : u32
        let s_3_0: u32 = 2;
        // D s_3_1: write-var domain <= s_3_0
        fn_state.domain = s_3_0;
        // C s_3_2: const #0u : u32
        let s_3_2: u32 = 0;
        // D s_3_3: write-var types <= s_3_2
        fn_state.types = s_3_2;
        // N s_3_4: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var types:u32
        let s_4_0: u32 = fn_state.types;
        // D s_4_1: write-var typesshadow#7148 <= s_4_0
        fn_state.typesshadow_7148 = s_4_0;
        // C s_4_2: const #16975u : u32
        let s_4_2: u32 = 16975;
        // D s_4_3: read-reg s_4_2:u8
        let s_4_3: u8 = {
            let value = state.read_register::<u8>(s_4_2 as isize);
            tracer.read_register(s_4_2 as isize, value);
            value
        };
        // D s_4_4: cast zx s_4_3 -> bv
        let s_4_4: Bits = Bits::new(s_4_3 as u128, 2u16);
        // C s_4_5: const #448u : u32
        let s_4_5: u32 = 448;
        // D s_4_6: read-reg s_4_5:u8
        let s_4_6: u8 = {
            let value = state.read_register::<u8>(s_4_5 as isize);
            tracer.read_register(s_4_5 as isize, value);
            value
        };
        // D s_4_7: cast zx s_4_6 -> bv
        let s_4_7: Bits = Bits::new(s_4_6 as u128, 2u16);
        // D s_4_8: cmp-eq s_4_4 s_4_7
        let s_4_8: bool = ((s_4_4) == (s_4_7));
        // N s_4_9: branch s_4_8 b28 b5
        if s_4_8 {
            return block_28(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #16975u : u32
        let s_5_0: u32 = 16975;
        // D s_5_1: read-reg s_5_0:u8
        let s_5_1: u8 = {
            let value = state.read_register::<u8>(s_5_0 as isize);
            tracer.read_register(s_5_0 as isize, value);
            value
        };
        // D s_5_2: cast zx s_5_1 -> bv
        let s_5_2: Bits = Bits::new(s_5_1 as u128, 2u16);
        // C s_5_3: const #440u : u32
        let s_5_3: u32 = 440;
        // D s_5_4: read-reg s_5_3:u8
        let s_5_4: u8 = {
            let value = state.read_register::<u8>(s_5_3 as isize);
            tracer.read_register(s_5_3 as isize, value);
            value
        };
        // D s_5_5: cast zx s_5_4 -> bv
        let s_5_5: Bits = Bits::new(s_5_4 as u128, 2u16);
        // D s_5_6: cmp-eq s_5_2 s_5_5
        let s_5_6: bool = ((s_5_2) == (s_5_5));
        // D s_5_7: write-var gs#296577 <= s_5_6
        fn_state.gs_296577 = s_5_6;
        // N s_5_8: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var gs#296577:u8
        let s_6_0: bool = fn_state.gs_296577;
        // N s_6_1: branch s_6_0 b27 b7
        if s_6_0 {
            return block_27(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #0u : u8
        let s_7_0: bool = false;
        // D s_7_1: write-var gs#296578 <= s_7_0
        fn_state.gs_296578 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var gs#296578:u8
        let s_8_0: bool = fn_state.gs_296578;
        // N s_8_1: branch s_8_0 b11 b9
        if s_8_0 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_9_0: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var domain:u32
        let s_10_0: u32 = fn_state.domain;
        // D s_10_1: read-var typesshadow#7148:u32
        let s_10_1: u32 = fn_state.typesshadow_7148;
        // D s_10_2: read-var nXS:u8
        let s_10_2: bool = fn_state.nXS;
        // D s_10_3: call DataSynchronizationBarrier(s_10_0, s_10_1, s_10_2)
        let s_10_3: () = DataSynchronizationBarrier(
            state,
            tracer,
            s_10_0,
            s_10_1,
            s_10_2,
        );
        // N s_10_4: return
        return;
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
        // C s_11_4: const #3u : u8
        let s_11_4: u8 = 3;
        // C s_11_5: cast zx s_11_4 -> bv
        let s_11_5: Bits = Bits::new(s_11_4 as u128, 2u16);
        // S s_11_6: cmp-eq s_11_3 s_11_5
        let s_11_6: bool = ((s_11_3) == (s_11_5));
        // N s_11_7: branch s_11_6 b26 b12
        if s_11_6 {
            return block_26(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_12_0: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #() : ()
        let s_13_0: () = ();
        // S s_13_1: call HCR_read(s_13_0)
        let s_13_1: ProductType700c18a878c5601b = HCR_read(state, tracer, s_13_0);
        // S s_13_2: call _get_HCR_Type_BSU(s_13_1)
        let s_13_2: u8 = u_get_HCR_Type_BSU(state, tracer, s_13_1);
        // S s_13_3: cast zx s_13_2 -> bv
        let s_13_3: Bits = Bits::new(s_13_2 as u128, 2u16);
        // C s_13_4: const #2u : u8
        let s_13_4: u8 = 2;
        // C s_13_5: cast zx s_13_4 -> bv
        let s_13_5: Bits = Bits::new(s_13_4 as u128, 2u16);
        // S s_13_6: cmp-eq s_13_3 s_13_5
        let s_13_6: bool = ((s_13_3) == (s_13_5));
        // N s_13_7: branch s_13_6 b25 b14
        if s_13_6 {
            return block_25(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #0u : u8
        let s_14_0: bool = false;
        // D s_14_1: write-var gs#296580 <= s_14_0
        fn_state.gs_296580 = s_14_0;
        // N s_14_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var gs#296580:u8
        let s_15_0: bool = fn_state.gs_296580;
        // N s_15_1: branch s_15_0 b24 b16
        if s_15_0 {
            return block_24(state, tracer, fn_state);
        } else {
            return block_16(state, tracer, fn_state);
        };
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_16_0: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_17_0: const #() : ()
        let s_17_0: () = ();
        // S s_17_1: call HCR_read(s_17_0)
        let s_17_1: ProductType700c18a878c5601b = HCR_read(state, tracer, s_17_0);
        // S s_17_2: call _get_HCR_Type_BSU(s_17_1)
        let s_17_2: u8 = u_get_HCR_Type_BSU(state, tracer, s_17_1);
        // S s_17_3: cast zx s_17_2 -> bv
        let s_17_3: Bits = Bits::new(s_17_2 as u128, 2u16);
        // C s_17_4: const #1u : u8
        let s_17_4: u8 = 1;
        // C s_17_5: cast zx s_17_4 -> bv
        let s_17_5: Bits = Bits::new(s_17_4 as u128, 2u16);
        // S s_17_6: cmp-eq s_17_3 s_17_5
        let s_17_6: bool = ((s_17_3) == (s_17_5));
        // N s_17_7: branch s_17_6 b23 b18
        if s_17_6 {
            return block_23(state, tracer, fn_state);
        } else {
            return block_18(state, tracer, fn_state);
        };
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_18_0: const #0u : u8
        let s_18_0: bool = false;
        // D s_18_1: write-var gs#296581 <= s_18_0
        fn_state.gs_296581 = s_18_0;
        // N s_18_2: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var gs#296581:u8
        let s_19_0: bool = fn_state.gs_296581;
        // N s_19_1: branch s_19_0 b22 b20
        if s_19_0 {
            return block_22(state, tracer, fn_state);
        } else {
            return block_20(state, tracer, fn_state);
        };
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_20_0: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_21_0: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_22_0: const #1u : u32
        let s_22_0: u32 = 1;
        // D s_22_1: write-var domain <= s_22_0
        fn_state.domain = s_22_0;
        // N s_22_2: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_23_0: read-var domain:u32
        let s_23_0: u32 = fn_state.domain;
        // C s_23_1: const #0u : u32
        let s_23_1: u32 = 0;
        // D s_23_2: cmp-eq s_23_0 s_23_1
        let s_23_2: bool = ((s_23_0) == (s_23_1));
        // D s_23_3: write-var gs#296581 <= s_23_2
        fn_state.gs_296581 = s_23_2;
        // N s_23_4: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_24_0: const #2u : u32
        let s_24_0: u32 = 2;
        // D s_24_1: write-var domain <= s_24_0
        fn_state.domain = s_24_0;
        // N s_24_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_25_0: read-var domain:u32
        let s_25_0: u32 = fn_state.domain;
        // C s_25_1: const #3u : u32
        let s_25_1: u32 = 3;
        // D s_25_2: cmp-eq s_25_0 s_25_1
        let s_25_2: bool = ((s_25_0) == (s_25_1));
        // D s_25_3: write-var gs#296580 <= s_25_2
        fn_state.gs_296580 = s_25_2;
        // N s_25_4: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_26_0: const #3u : u32
        let s_26_0: u32 = 3;
        // D s_26_1: write-var domain <= s_26_0
        fn_state.domain = s_26_0;
        // N s_26_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_27_0: const #() : ()
        let s_27_0: () = ();
        // S s_27_1: call EL2Enabled(s_27_0)
        let s_27_1: bool = EL2Enabled(state, tracer, s_27_0);
        // D s_27_2: write-var gs#296578 <= s_27_1
        fn_state.gs_296578 = s_27_1;
        // N s_27_3: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_28_0: const #1u : u8
        let s_28_0: bool = true;
        // D s_28_1: write-var gs#296577 <= s_28_0
        fn_state.gs_296577 = s_28_0;
        // N s_28_2: jump b6
        return block_6(state, tracer, fn_state);
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
        // C s_29_2: const #2u : u8
        let s_29_2: u8 = 2;
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
        // C s_30_2: const #1u : u32
        let s_30_2: u32 = 1;
        // D s_30_3: write-var types <= s_30_2
        fn_state.types = s_30_2;
        // N s_30_4: jump b4
        return block_4(state, tracer, fn_state);
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
        // C s_31_2: const #3u : u8
        let s_31_2: u8 = 3;
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
        // C s_32_0: const #2u : u32
        let s_32_0: u32 = 2;
        // D s_32_1: write-var domain <= s_32_0
        fn_state.domain = s_32_0;
        // C s_32_2: const #2u : u32
        let s_32_2: u32 = 2;
        // D s_32_3: write-var types <= s_32_2
        fn_state.types = s_32_2;
        // N s_32_4: jump b4
        return block_4(state, tracer, fn_state);
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
        // C s_33_2: const #5u : u8
        let s_33_2: u8 = 5;
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
        // C s_34_2: const #0u : u32
        let s_34_2: u32 = 0;
        // D s_34_3: write-var types <= s_34_2
        fn_state.types = s_34_2;
        // N s_34_4: jump b4
        return block_4(state, tracer, fn_state);
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
        // C s_35_2: const #6u : u8
        let s_35_2: u8 = 6;
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
        // C s_36_2: const #1u : u32
        let s_36_2: u32 = 1;
        // D s_36_3: write-var types <= s_36_2
        fn_state.types = s_36_2;
        // N s_36_4: jump b4
        return block_4(state, tracer, fn_state);
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
        // C s_37_2: const #7u : u8
        let s_37_2: u8 = 7;
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
        // C s_38_0: const #0u : u32
        let s_38_0: u32 = 0;
        // D s_38_1: write-var domain <= s_38_0
        fn_state.domain = s_38_0;
        // C s_38_2: const #2u : u32
        let s_38_2: u32 = 2;
        // D s_38_3: write-var types <= s_38_2
        fn_state.types = s_38_2;
        // N s_38_4: jump b4
        return block_4(state, tracer, fn_state);
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
        // C s_39_2: const #9u : u8
        let s_39_2: u8 = 9;
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
        // C s_40_2: const #0u : u32
        let s_40_2: u32 = 0;
        // D s_40_3: write-var types <= s_40_2
        fn_state.types = s_40_2;
        // N s_40_4: jump b4
        return block_4(state, tracer, fn_state);
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
        // C s_41_2: const #10u : u8
        let s_41_2: u8 = 10;
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
        // C s_42_2: const #1u : u32
        let s_42_2: u32 = 1;
        // D s_42_3: write-var types <= s_42_2
        fn_state.types = s_42_2;
        // N s_42_4: jump b4
        return block_4(state, tracer, fn_state);
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
        // C s_43_2: const #11u : u8
        let s_43_2: u8 = 11;
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
        // C s_44_0: const #1u : u32
        let s_44_0: u32 = 1;
        // D s_44_1: write-var domain <= s_44_0
        fn_state.domain = s_44_0;
        // C s_44_2: const #2u : u32
        let s_44_2: u32 = 2;
        // D s_44_3: write-var types <= s_44_2
        fn_state.types = s_44_2;
        // N s_44_4: jump b4
        return block_4(state, tracer, fn_state);
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
        // C s_45_2: const #13u : u8
        let s_45_2: u8 = 13;
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
        // C s_46_2: const #0u : u32
        let s_46_2: u32 = 0;
        // D s_46_3: write-var types <= s_46_2
        fn_state.types = s_46_2;
        // N s_46_4: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_47_0: read-var option_name:u8
        let s_47_0: u8 = fn_state.option_name;
        // D s_47_1: cast zx s_47_0 -> bv
        let s_47_1: Bits = Bits::new(s_47_0 as u128, 4u16);
        // C s_47_2: const #14u : u8
        let s_47_2: u8 = 14;
        // C s_47_3: cast zx s_47_2 -> bv
        let s_47_3: Bits = Bits::new(s_47_2 as u128, 4u16);
        // D s_47_4: cmp-eq s_47_1 s_47_3
        let s_47_4: bool = ((s_47_1) == (s_47_3));
        // D s_47_5: not s_47_4
        let s_47_5: bool = !s_47_4;
        // N s_47_6: branch s_47_5 b49 b48
        if s_47_5 {
            return block_49(state, tracer, fn_state);
        } else {
            return block_48(state, tracer, fn_state);
        };
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_48_0: const #3u : u32
        let s_48_0: u32 = 3;
        // D s_48_1: write-var domain <= s_48_0
        fn_state.domain = s_48_0;
        // C s_48_2: const #1u : u32
        let s_48_2: u32 = 1;
        // D s_48_3: write-var types <= s_48_2
        fn_state.types = s_48_2;
        // N s_48_4: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_49_0: read-var option_name:u8
        let s_49_0: u8 = fn_state.option_name;
        // D s_49_1: write-var b__11 <= s_49_0
        fn_state.b__11 = s_49_0;
        // C s_49_2: const #3s : i
        let s_49_2: i128 = 3;
        // D s_49_3: read-var b__11:u8
        let s_49_3: u8 = fn_state.b__11;
        // D s_49_4: cast zx s_49_3 -> bv
        let s_49_4: Bits = Bits::new(s_49_3 as u128, 4u16);
        // C s_49_5: const #1s : i64
        let s_49_5: i64 = 1;
        // C s_49_6: cast zx s_49_5 -> i
        let s_49_6: i128 = (i128::try_from(s_49_5).unwrap());
        // C s_49_7: const #0s : i
        let s_49_7: i128 = 0;
        // C s_49_8: add s_49_7 s_49_6
        let s_49_8: i128 = (s_49_7 + s_49_6);
        // D s_49_9: bit-extract s_49_4 s_49_2 s_49_8
        let s_49_9: Bits = (Bits::new(
            ((s_49_4) >> (s_49_2)).value(),
            u16::try_from(s_49_8).unwrap(),
        ));
        // D s_49_10: cast reint s_49_9 -> u8
        let s_49_10: bool = ((s_49_9.value()) != 0);
        // D s_49_11: cast zx s_49_10 -> bv
        let s_49_11: Bits = Bits::new(s_49_10 as u128, 1u16);
        // C s_49_12: const #0u : u8
        let s_49_12: bool = false;
        // C s_49_13: cast zx s_49_12 -> bv
        let s_49_13: Bits = Bits::new(s_49_12 as u128, 1u16);
        // D s_49_14: cmp-eq s_49_11 s_49_13
        let s_49_14: bool = ((s_49_11) == (s_49_13));
        // N s_49_15: branch s_49_14 b55 b50
        if s_49_14 {
            return block_55(state, tracer, fn_state);
        } else {
            return block_50(state, tracer, fn_state);
        };
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_50_0: const #0u : u8
        let s_50_0: bool = false;
        // D s_50_1: write-var gs#296570 <= s_50_0
        fn_state.gs_296570 = s_50_0;
        // N s_50_2: jump b51
        return block_51(state, tracer, fn_state);
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_51_0: read-var gs#296570:u8
        let s_51_0: bool = fn_state.gs_296570;
        // D s_51_1: not s_51_0
        let s_51_1: bool = !s_51_0;
        // N s_51_2: branch s_51_1 b54 b52
        if s_51_1 {
            return block_54(state, tracer, fn_state);
        } else {
            return block_52(state, tracer, fn_state);
        };
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_52_0: const #1u : u8
        let s_52_0: bool = true;
        // D s_52_1: write-var gs#296565 <= s_52_0
        fn_state.gs_296565 = s_52_0;
        // N s_52_2: jump b53
        return block_53(state, tracer, fn_state);
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_53_0: read-var gs#296565:u8
        let s_53_0: bool = fn_state.gs_296565;
        // D s_53_1: not s_53_0
        let s_53_1: bool = !s_53_0;
        // N s_53_2: assert s_53_1
        let s_53_2: () = assert!(s_53_1);
        // C s_53_3: const #3u : u32
        let s_53_3: u32 = 3;
        // D s_53_4: write-var domain <= s_53_3
        fn_state.domain = s_53_3;
        // C s_53_5: const #2u : u32
        let s_53_5: u32 = 2;
        // D s_53_6: write-var types <= s_53_5
        fn_state.types = s_53_5;
        // N s_53_7: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_54_0: const #0u : u8
        let s_54_0: bool = false;
        // D s_54_1: write-var gs#296565 <= s_54_0
        fn_state.gs_296565 = s_54_0;
        // N s_54_2: jump b53
        return block_53(state, tracer, fn_state);
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_55_0: const #0s : i
        let s_55_0: i128 = 0;
        // D s_55_1: read-var b__11:u8
        let s_55_1: u8 = fn_state.b__11;
        // D s_55_2: cast zx s_55_1 -> bv
        let s_55_2: Bits = Bits::new(s_55_1 as u128, 4u16);
        // C s_55_3: const #1s : i64
        let s_55_3: i64 = 1;
        // C s_55_4: cast zx s_55_3 -> i
        let s_55_4: i128 = (i128::try_from(s_55_3).unwrap());
        // C s_55_5: const #1s : i
        let s_55_5: i128 = 1;
        // C s_55_6: add s_55_5 s_55_4
        let s_55_6: i128 = (s_55_5 + s_55_4);
        // D s_55_7: bit-extract s_55_2 s_55_0 s_55_6
        let s_55_7: Bits = (Bits::new(
            ((s_55_2) >> (s_55_0)).value(),
            u16::try_from(s_55_6).unwrap(),
        ));
        // D s_55_8: cast reint s_55_7 -> u8
        let s_55_8: u8 = (s_55_7.value() as u8);
        // D s_55_9: cast zx s_55_8 -> bv
        let s_55_9: Bits = Bits::new(s_55_8 as u128, 2u16);
        // C s_55_10: const #0u : u8
        let s_55_10: u8 = 0;
        // C s_55_11: cast zx s_55_10 -> bv
        let s_55_11: Bits = Bits::new(s_55_10 as u128, 2u16);
        // D s_55_12: cmp-eq s_55_9 s_55_11
        let s_55_12: bool = ((s_55_9) == (s_55_11));
        // D s_55_13: write-var gs#296570 <= s_55_12
        fn_state.gs_296570 = s_55_12;
        // N s_55_14: jump b51
        return block_51(state, tracer, fn_state);
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_56_0: const #16975u : u32
        let s_56_0: u32 = 16975;
        // D s_56_1: read-reg s_56_0:u8
        let s_56_1: u8 = {
            let value = state.read_register::<u8>(s_56_0 as isize);
            tracer.read_register(s_56_0 as isize, value);
            value
        };
        // D s_56_2: cast zx s_56_1 -> bv
        let s_56_2: Bits = Bits::new(s_56_1 as u128, 2u16);
        // C s_56_3: const #448u : u32
        let s_56_3: u32 = 448;
        // D s_56_4: read-reg s_56_3:u8
        let s_56_4: u8 = {
            let value = state.read_register::<u8>(s_56_3 as isize);
            tracer.read_register(s_56_3 as isize, value);
            value
        };
        // D s_56_5: cast zx s_56_4 -> bv
        let s_56_5: Bits = Bits::new(s_56_4 as u128, 2u16);
        // D s_56_6: cmp-eq s_56_2 s_56_5
        let s_56_6: bool = ((s_56_2) == (s_56_5));
        // N s_56_7: branch s_56_6 b68 b57
        if s_56_6 {
            return block_68(state, tracer, fn_state);
        } else {
            return block_57(state, tracer, fn_state);
        };
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_57_0: const #16975u : u32
        let s_57_0: u32 = 16975;
        // D s_57_1: read-reg s_57_0:u8
        let s_57_1: u8 = {
            let value = state.read_register::<u8>(s_57_0 as isize);
            tracer.read_register(s_57_0 as isize, value);
            value
        };
        // D s_57_2: cast zx s_57_1 -> bv
        let s_57_2: Bits = Bits::new(s_57_1 as u128, 2u16);
        // C s_57_3: const #440u : u32
        let s_57_3: u32 = 440;
        // D s_57_4: read-reg s_57_3:u8
        let s_57_4: u8 = {
            let value = state.read_register::<u8>(s_57_3 as isize);
            tracer.read_register(s_57_3 as isize, value);
            value
        };
        // D s_57_5: cast zx s_57_4 -> bv
        let s_57_5: Bits = Bits::new(s_57_4 as u128, 2u16);
        // D s_57_6: cmp-eq s_57_2 s_57_5
        let s_57_6: bool = ((s_57_2) == (s_57_5));
        // D s_57_7: write-var gs#296538 <= s_57_6
        fn_state.gs_296538 = s_57_6;
        // N s_57_8: jump b58
        return block_58(state, tracer, fn_state);
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_58_0: read-var gs#296538:u8
        let s_58_0: bool = fn_state.gs_296538;
        // N s_58_1: branch s_58_0 b67 b59
        if s_58_0 {
            return block_67(state, tracer, fn_state);
        } else {
            return block_59(state, tracer, fn_state);
        };
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_59_0: const #0u : u8
        let s_59_0: bool = false;
        // D s_59_1: write-var gs#296539 <= s_59_0
        fn_state.gs_296539 = s_59_0;
        // N s_59_2: jump b60
        return block_60(state, tracer, fn_state);
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_60_0: read-var gs#296539:u8
        let s_60_0: bool = fn_state.gs_296539;
        // N s_60_1: branch s_60_0 b66 b61
        if s_60_0 {
            return block_66(state, tracer, fn_state);
        } else {
            return block_61(state, tracer, fn_state);
        };
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_61_0: const #0u : u8
        let s_61_0: bool = false;
        // D s_61_1: write-var gs#296540 <= s_61_0
        fn_state.gs_296540 = s_61_0;
        // N s_61_2: jump b62
        return block_62(state, tracer, fn_state);
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_62_0: read-var gs#296540:u8
        let s_62_0: bool = fn_state.gs_296540;
        // N s_62_1: branch s_62_0 b65 b63
        if s_62_0 {
            return block_65(state, tracer, fn_state);
        } else {
            return block_63(state, tracer, fn_state);
        };
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_63_0: const #0u : u8
        let s_63_0: bool = false;
        // D s_63_1: write-var gs#296541 <= s_63_0
        fn_state.gs_296541 = s_63_0;
        // N s_63_2: jump b64
        return block_64(state, tracer, fn_state);
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_64_0: read-var gs#296541:u8
        let s_64_0: bool = fn_state.gs_296541;
        // D s_64_1: write-var nXS <= s_64_0
        fn_state.nXS = s_64_0;
        // N s_64_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_65<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_65_0: const #22528u : u32
        let s_65_0: u32 = 22528;
        // D s_65_1: read-reg s_65_0:struct
        let s_65_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_65_0 as isize);
            tracer.read_register(s_65_0 as isize, value);
            value
        };
        // D s_65_2: call _get_HCRX_EL2_Type_FnXS(s_65_1)
        let s_65_2: bool = u_get_HCRX_EL2_Type_FnXS(state, tracer, s_65_1);
        // D s_65_3: cast zx s_65_2 -> bv
        let s_65_3: Bits = Bits::new(s_65_2 as u128, 1u16);
        // C s_65_4: const #1u : u8
        let s_65_4: bool = true;
        // C s_65_5: cast zx s_65_4 -> bv
        let s_65_5: Bits = Bits::new(s_65_4 as u128, 1u16);
        // D s_65_6: cmp-eq s_65_3 s_65_5
        let s_65_6: bool = ((s_65_3) == (s_65_5));
        // D s_65_7: write-var gs#296541 <= s_65_6
        fn_state.gs_296541 = s_65_6;
        // N s_65_8: jump b64
        return block_64(state, tracer, fn_state);
    }
    fn block_66<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_66_0: const #() : ()
        let s_66_0: () = ();
        // S s_66_1: call IsHCRXEL2Enabled(s_66_0)
        let s_66_1: bool = IsHCRXEL2Enabled(state, tracer, s_66_0);
        // D s_66_2: write-var gs#296540 <= s_66_1
        fn_state.gs_296540 = s_66_1;
        // N s_66_3: jump b62
        return block_62(state, tracer, fn_state);
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_67_0: const #432u : u32
        let s_67_0: u32 = 432;
        // D s_67_1: read-reg s_67_0:u8
        let s_67_1: u8 = {
            let value = state.read_register::<u8>(s_67_0 as isize);
            tracer.read_register(s_67_0 as isize, value);
            value
        };
        // D s_67_2: call ELUsingAArch32(s_67_1)
        let s_67_2: bool = ELUsingAArch32(state, tracer, s_67_1);
        // D s_67_3: not s_67_2
        let s_67_3: bool = !s_67_2;
        // D s_67_4: write-var gs#296539 <= s_67_3
        fn_state.gs_296539 = s_67_3;
        // N s_67_5: jump b60
        return block_60(state, tracer, fn_state);
    }
    fn block_68<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_68_0: const #1u : u8
        let s_68_0: bool = true;
        // D s_68_1: write-var gs#296538 <= s_68_0
        fn_state.gs_296538 = s_68_0;
        // N s_68_2: jump b58
        return block_58(state, tracer, fn_state);
    }
}
