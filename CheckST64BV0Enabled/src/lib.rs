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
use EL3SDDUndefPriority::*;
use IsInHost::*;
use IsHCRXEL2Enabled::*;
use u_get_SCR_EL3_Type_EnAS0::*;
use u_get_HCRX_EL2_Type_EnAS0::*;
use u_get_SCTLR_EL2_Type_EnAS0::*;
use u_get_SCTLR_EL1_Type_EnAS0::*;
use EL3SDDUndef::*;
use EL2Enabled::*;
use u_get_HCR_EL2_Type_TGE::*;
use LDST64BTrap::*;
use common::*;
pub fn CheckST64BV0Enabled<T: Tracer>(
    state: &mut State,
    tracer: &T,
    gs_24698: (),
) -> () {
    #[derive(Default)]
    struct FunctionState {
        trap: bool,
        gs_24703: bool,
        iss: u32,
        gs_24706: bool,
        gs_24719: bool,
        gs_24704: bool,
        gs_24705: bool,
        gs_24702: bool,
        gs_24701: bool,
        gs_24707: bool,
        gs_24714: bool,
        gs_24711: bool,
        gs_24700: bool,
        target_el: u8,
        gs_24709: bool,
        gs_24698: (),
    }
    let fn_state = FunctionState {
        gs_24698,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #0u : u8
        let s_0_0: bool = false;
        // D s_0_1: write-var trap <= s_0_0
        fn_state.trap = s_0_0;
        // C s_0_2: const #25s : i
        let s_0_2: i128 = 25;
        // C s_0_3: const #1u : u8
        let s_0_3: bool = true;
        // C s_0_4: cast zx s_0_3 -> bv
        let s_0_4: Bits = Bits::new(s_0_3 as u128, 1u16);
        // D s_0_5: bits-cast zx s_0_4 -> bv length s_0_2
        let s_0_5: Bits = s_0_4.zero_extend(s_0_2);
        // D s_0_6: cast reint s_0_5 -> u25
        let s_0_6: u32 = (s_0_5.value() as u32);
        // D s_0_7: write-var iss <= s_0_6
        fn_state.iss = s_0_6;
        // C s_0_8: const #16975u : u32
        let s_0_8: u32 = 16975;
        // D s_0_9: read-reg s_0_8:u8
        let s_0_9: u8 = {
            let value = state.read_register::<u8>(s_0_8 as isize);
            tracer.read_register(s_0_8 as isize, value);
            value
        };
        // D s_0_10: cast zx s_0_9 -> bv
        let s_0_10: Bits = Bits::new(s_0_9 as u128, 2u16);
        // C s_0_11: const #424u : u32
        let s_0_11: u32 = 424;
        // D s_0_12: read-reg s_0_11:u8
        let s_0_12: u8 = {
            let value = state.read_register::<u8>(s_0_11 as isize);
            tracer.read_register(s_0_11 as isize, value);
            value
        };
        // D s_0_13: cast zx s_0_12 -> bv
        let s_0_13: Bits = Bits::new(s_0_12 as u128, 2u16);
        // D s_0_14: cmp-ne s_0_10 s_0_13
        let s_0_14: bool = ((s_0_10) != (s_0_13));
        // N s_0_15: branch s_0_14 b56 b1
        if s_0_14 {
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
        // D s_1_1: write-var gs#24700 <= s_1_0
        fn_state.gs_24700 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#24700:u8
        let s_2_0: bool = fn_state.gs_24700;
        // N s_2_1: branch s_2_0 b55 b3
        if s_2_0 {
            return block_55(state, tracer, fn_state);
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
        // D s_3_1: write-var gs#24701 <= s_3_0
        fn_state.gs_24701 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#24701:u8
        let s_4_0: bool = fn_state.gs_24701;
        // N s_4_1: branch s_4_0 b54 b5
        if s_4_0 {
            return block_54(state, tracer, fn_state);
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
        // D s_5_1: write-var gs#24702 <= s_5_0
        fn_state.gs_24702 = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var gs#24702:u8
        let s_6_0: bool = fn_state.gs_24702;
        // N s_6_1: branch s_6_0 b53 b7
        if s_6_0 {
            return block_53(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #16975u : u32
        let s_7_0: u32 = 16975;
        // D s_7_1: read-reg s_7_0:u8
        let s_7_1: u8 = {
            let value = state.read_register::<u8>(s_7_0 as isize);
            tracer.read_register(s_7_0 as isize, value);
            value
        };
        // D s_7_2: cast zx s_7_1 -> bv
        let s_7_2: Bits = Bits::new(s_7_1 as u128, 2u16);
        // C s_7_3: const #448u : u32
        let s_7_3: u32 = 448;
        // D s_7_4: read-reg s_7_3:u8
        let s_7_4: u8 = {
            let value = state.read_register::<u8>(s_7_3 as isize);
            tracer.read_register(s_7_3 as isize, value);
            value
        };
        // D s_7_5: cast zx s_7_4 -> bv
        let s_7_5: Bits = Bits::new(s_7_4 as u128, 2u16);
        // D s_7_6: cmp-eq s_7_2 s_7_5
        let s_7_6: bool = ((s_7_2) == (s_7_5));
        // N s_7_7: branch s_7_6 b44 b8
        if s_7_6 {
            return block_44(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_8_0: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var trap:u8
        let s_9_0: bool = fn_state.trap;
        // D s_9_1: not s_9_0
        let s_9_1: bool = !s_9_0;
        // N s_9_2: branch s_9_1 b43 b10
        if s_9_1 {
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
        // D s_10_1: write-var gs#24703 <= s_10_0
        fn_state.gs_24703 = s_10_0;
        // N s_10_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var gs#24703:u8
        let s_11_0: bool = fn_state.gs_24703;
        // N s_11_1: branch s_11_0 b36 b12
        if s_11_0 {
            return block_36(state, tracer, fn_state);
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
        // D s_12_1: write-var gs#24706 <= s_12_0
        fn_state.gs_24706 = s_12_0;
        // N s_12_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var gs#24706:u8
        let s_13_0: bool = fn_state.gs_24706;
        // N s_13_1: branch s_13_0 b32 b14
        if s_13_0 {
            return block_32(state, tracer, fn_state);
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
        // D s_15_0: read-var trap:u8
        let s_15_0: bool = fn_state.trap;
        // D s_15_1: not s_15_0
        let s_15_1: bool = !s_15_0;
        // N s_15_2: branch s_15_1 b31 b16
        if s_15_1 {
            return block_31(state, tracer, fn_state);
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
        // D s_16_1: write-var gs#24707 <= s_16_0
        fn_state.gs_24707 = s_16_0;
        // N s_16_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var gs#24707:u8
        let s_17_0: bool = fn_state.gs_24707;
        // N s_17_1: branch s_17_0 b27 b18
        if s_17_0 {
            return block_27(state, tracer, fn_state);
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
        // D s_19_0: read-var trap:u8
        let s_19_0: bool = fn_state.trap;
        // N s_19_1: branch s_19_0 b21 b20
        if s_19_0 {
            return block_21(state, tracer, fn_state);
        } else {
            return block_20(state, tracer, fn_state);
        };
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_20_0: return
        return;
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var target_el:u8
        let s_21_0: u8 = fn_state.target_el;
        // D s_21_1: cast zx s_21_0 -> bv
        let s_21_1: Bits = Bits::new(s_21_0 as u128, 2u16);
        // C s_21_2: const #424u : u32
        let s_21_2: u32 = 424;
        // D s_21_3: read-reg s_21_2:u8
        let s_21_3: u8 = {
            let value = state.read_register::<u8>(s_21_2 as isize);
            tracer.read_register(s_21_2 as isize, value);
            value
        };
        // D s_21_4: cast zx s_21_3 -> bv
        let s_21_4: Bits = Bits::new(s_21_3 as u128, 2u16);
        // D s_21_5: cmp-eq s_21_1 s_21_4
        let s_21_5: bool = ((s_21_1) == (s_21_4));
        // N s_21_6: branch s_21_5 b26 b22
        if s_21_5 {
            return block_26(state, tracer, fn_state);
        } else {
            return block_22(state, tracer, fn_state);
        };
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_22_0: const #0u : u8
        let s_22_0: bool = false;
        // D s_22_1: write-var gs#24709 <= s_22_0
        fn_state.gs_24709 = s_22_0;
        // N s_22_2: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_23_0: read-var gs#24709:u8
        let s_23_0: bool = fn_state.gs_24709;
        // N s_23_1: branch s_23_0 b25 b24
        if s_23_0 {
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
        // D s_24_0: read-var target_el:u8
        let s_24_0: u8 = fn_state.target_el;
        // D s_24_1: read-var iss:u25
        let s_24_1: u32 = fn_state.iss;
        // D s_24_2: call LDST64BTrap(s_24_0, s_24_1)
        let s_24_2: () = LDST64BTrap(state, tracer, s_24_0, s_24_1);
        // N s_24_3: return
        return;
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_25_0: panic
        panic!("{:?}", ());
        // N s_25_1: return
        return;
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_26_0: const #() : ()
        let s_26_0: () = ();
        // S s_26_1: call EL3SDDUndef(s_26_0)
        let s_26_1: bool = EL3SDDUndef(state, tracer, s_26_0);
        // D s_26_2: write-var gs#24709 <= s_26_1
        fn_state.gs_24709 = s_26_1;
        // N s_26_3: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_27_0: const #424u : u32
        let s_27_0: u32 = 424;
        // D s_27_1: read-reg s_27_0:u8
        let s_27_1: u8 = {
            let value = state.read_register::<u8>(s_27_0 as isize);
            tracer.read_register(s_27_0 as isize, value);
            value
        };
        // C s_27_2: const #2u : u8
        let s_27_2: u8 = 2;
        // D s_27_3: cmp-lt s_27_1 s_27_2
        let s_27_3: bool = ((s_27_1) < (s_27_2));
        // N s_27_4: branch s_27_3 b30 b28
        if s_27_3 {
            return block_30(state, tracer, fn_state);
        } else {
            return block_28(state, tracer, fn_state);
        };
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_28_0: const #0u : u8
        let s_28_0: bool = false;
        // D s_28_1: write-var gs#24711 <= s_28_0
        fn_state.gs_24711 = s_28_0;
        // N s_28_2: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_29_0: read-var gs#24711:u8
        let s_29_0: bool = fn_state.gs_24711;
        // D s_29_1: write-var trap <= s_29_0
        fn_state.trap = s_29_0;
        // C s_29_2: const #424u : u32
        let s_29_2: u32 = 424;
        // D s_29_3: read-reg s_29_2:u8
        let s_29_3: u8 = {
            let value = state.read_register::<u8>(s_29_2 as isize);
            tracer.read_register(s_29_2 as isize, value);
            value
        };
        // D s_29_4: write-var target_el <= s_29_3
        fn_state.target_el = s_29_3;
        // N s_29_5: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_30_0: const #90704u : u32
        let s_30_0: u32 = 90704;
        // D s_30_1: read-reg s_30_0:struct
        let s_30_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_30_0 as isize);
            tracer.read_register(s_30_0 as isize, value);
            value
        };
        // D s_30_2: call _get_SCR_EL3_Type_EnAS0(s_30_1)
        let s_30_2: bool = u_get_SCR_EL3_Type_EnAS0(state, tracer, s_30_1);
        // D s_30_3: cast zx s_30_2 -> bv
        let s_30_3: Bits = Bits::new(s_30_2 as u128, 1u16);
        // C s_30_4: const #0u : u8
        let s_30_4: bool = false;
        // C s_30_5: cast zx s_30_4 -> bv
        let s_30_5: Bits = Bits::new(s_30_4 as u128, 1u16);
        // D s_30_6: cmp-eq s_30_3 s_30_5
        let s_30_6: bool = ((s_30_3) == (s_30_5));
        // D s_30_7: write-var gs#24711 <= s_30_6
        fn_state.gs_24711 = s_30_6;
        // N s_30_8: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_31_0: const #16975u : u32
        let s_31_0: u32 = 16975;
        // D s_31_1: read-reg s_31_0:u8
        let s_31_1: u8 = {
            let value = state.read_register::<u8>(s_31_0 as isize);
            tracer.read_register(s_31_0 as isize, value);
            value
        };
        // D s_31_2: cast zx s_31_1 -> bv
        let s_31_2: Bits = Bits::new(s_31_1 as u128, 2u16);
        // C s_31_3: const #424u : u32
        let s_31_3: u32 = 424;
        // D s_31_4: read-reg s_31_3:u8
        let s_31_4: u8 = {
            let value = state.read_register::<u8>(s_31_3 as isize);
            tracer.read_register(s_31_3 as isize, value);
            value
        };
        // D s_31_5: cast zx s_31_4 -> bv
        let s_31_5: Bits = Bits::new(s_31_4 as u128, 2u16);
        // D s_31_6: cmp-ne s_31_2 s_31_5
        let s_31_6: bool = ((s_31_2) != (s_31_5));
        // D s_31_7: write-var gs#24707 <= s_31_6
        fn_state.gs_24707 = s_31_6;
        // N s_31_8: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_32_0: const #() : ()
        let s_32_0: () = ();
        // S s_32_1: call IsHCRXEL2Enabled(s_32_0)
        let s_32_1: bool = IsHCRXEL2Enabled(state, tracer, s_32_0);
        // S s_32_2: not s_32_1
        let s_32_2: bool = !s_32_1;
        // N s_32_3: branch s_32_2 b35 b33
        if s_32_2 {
            return block_35(state, tracer, fn_state);
        } else {
            return block_33(state, tracer, fn_state);
        };
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_33_0: const #22528u : u32
        let s_33_0: u32 = 22528;
        // D s_33_1: read-reg s_33_0:struct
        let s_33_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_33_0 as isize);
            tracer.read_register(s_33_0 as isize, value);
            value
        };
        // D s_33_2: call _get_HCRX_EL2_Type_EnAS0(s_33_1)
        let s_33_2: bool = u_get_HCRX_EL2_Type_EnAS0(state, tracer, s_33_1);
        // D s_33_3: cast zx s_33_2 -> bv
        let s_33_3: Bits = Bits::new(s_33_2 as u128, 1u16);
        // C s_33_4: const #0u : u8
        let s_33_4: bool = false;
        // C s_33_5: cast zx s_33_4 -> bv
        let s_33_5: Bits = Bits::new(s_33_4 as u128, 1u16);
        // D s_33_6: cmp-eq s_33_3 s_33_5
        let s_33_6: bool = ((s_33_3) == (s_33_5));
        // D s_33_7: write-var gs#24714 <= s_33_6
        fn_state.gs_24714 = s_33_6;
        // N s_33_8: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_34_0: read-var gs#24714:u8
        let s_34_0: bool = fn_state.gs_24714;
        // D s_34_1: write-var trap <= s_34_0
        fn_state.trap = s_34_0;
        // C s_34_2: const #432u : u32
        let s_34_2: u32 = 432;
        // D s_34_3: read-reg s_34_2:u8
        let s_34_3: u8 = {
            let value = state.read_register::<u8>(s_34_2 as isize);
            tracer.read_register(s_34_2 as isize, value);
            value
        };
        // D s_34_4: write-var target_el <= s_34_3
        fn_state.target_el = s_34_3;
        // N s_34_5: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_35_0: const #1u : u8
        let s_35_0: bool = true;
        // D s_35_1: write-var gs#24714 <= s_35_0
        fn_state.gs_24714 = s_35_0;
        // N s_35_2: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_36_0: const #16975u : u32
        let s_36_0: u32 = 16975;
        // D s_36_1: read-reg s_36_0:u8
        let s_36_1: u8 = {
            let value = state.read_register::<u8>(s_36_0 as isize);
            tracer.read_register(s_36_0 as isize, value);
            value
        };
        // D s_36_2: cast zx s_36_1 -> bv
        let s_36_2: Bits = Bits::new(s_36_1 as u128, 2u16);
        // C s_36_3: const #448u : u32
        let s_36_3: u32 = 448;
        // D s_36_4: read-reg s_36_3:u8
        let s_36_4: u8 = {
            let value = state.read_register::<u8>(s_36_3 as isize);
            tracer.read_register(s_36_3 as isize, value);
            value
        };
        // D s_36_5: cast zx s_36_4 -> bv
        let s_36_5: Bits = Bits::new(s_36_4 as u128, 2u16);
        // D s_36_6: cmp-eq s_36_2 s_36_5
        let s_36_6: bool = ((s_36_2) == (s_36_5));
        // N s_36_7: branch s_36_6 b42 b37
        if s_36_6 {
            return block_42(state, tracer, fn_state);
        } else {
            return block_37(state, tracer, fn_state);
        };
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_37_0: const #0u : u8
        let s_37_0: bool = false;
        // D s_37_1: write-var gs#24704 <= s_37_0
        fn_state.gs_24704 = s_37_0;
        // N s_37_2: jump b38
        return block_38(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_38_0: read-var gs#24704:u8
        let s_38_0: bool = fn_state.gs_24704;
        // N s_38_1: branch s_38_0 b41 b39
        if s_38_0 {
            return block_41(state, tracer, fn_state);
        } else {
            return block_39(state, tracer, fn_state);
        };
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_39_0: const #16975u : u32
        let s_39_0: u32 = 16975;
        // D s_39_1: read-reg s_39_0:u8
        let s_39_1: u8 = {
            let value = state.read_register::<u8>(s_39_0 as isize);
            tracer.read_register(s_39_0 as isize, value);
            value
        };
        // D s_39_2: cast zx s_39_1 -> bv
        let s_39_2: Bits = Bits::new(s_39_1 as u128, 2u16);
        // C s_39_3: const #440u : u32
        let s_39_3: u32 = 440;
        // D s_39_4: read-reg s_39_3:u8
        let s_39_4: u8 = {
            let value = state.read_register::<u8>(s_39_3 as isize);
            tracer.read_register(s_39_3 as isize, value);
            value
        };
        // D s_39_5: cast zx s_39_4 -> bv
        let s_39_5: Bits = Bits::new(s_39_4 as u128, 2u16);
        // D s_39_6: cmp-eq s_39_2 s_39_5
        let s_39_6: bool = ((s_39_2) == (s_39_5));
        // D s_39_7: write-var gs#24705 <= s_39_6
        fn_state.gs_24705 = s_39_6;
        // N s_39_8: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_40_0: read-var gs#24705:u8
        let s_40_0: bool = fn_state.gs_24705;
        // D s_40_1: write-var gs#24706 <= s_40_0
        fn_state.gs_24706 = s_40_0;
        // N s_40_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_41_0: const #1u : u8
        let s_41_0: bool = true;
        // D s_41_1: write-var gs#24705 <= s_41_0
        fn_state.gs_24705 = s_41_0;
        // N s_41_2: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_42_0: const #() : ()
        let s_42_0: () = ();
        // S s_42_1: call IsInHost(s_42_0)
        let s_42_1: bool = IsInHost(state, tracer, s_42_0);
        // S s_42_2: not s_42_1
        let s_42_2: bool = !s_42_1;
        // D s_42_3: write-var gs#24704 <= s_42_2
        fn_state.gs_24704 = s_42_2;
        // N s_42_4: jump b38
        return block_38(state, tracer, fn_state);
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_43_0: const #() : ()
        let s_43_0: () = ();
        // S s_43_1: call EL2Enabled(s_43_0)
        let s_43_1: bool = EL2Enabled(state, tracer, s_43_0);
        // D s_43_2: write-var gs#24703 <= s_43_1
        fn_state.gs_24703 = s_43_1;
        // N s_43_3: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_44_0: const #() : ()
        let s_44_0: () = ();
        // S s_44_1: call IsInHost(s_44_0)
        let s_44_1: bool = IsInHost(state, tracer, s_44_0);
        // S s_44_2: not s_44_1
        let s_44_2: bool = !s_44_1;
        // N s_44_3: branch s_44_2 b46 b45
        if s_44_2 {
            return block_46(state, tracer, fn_state);
        } else {
            return block_45(state, tracer, fn_state);
        };
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_45_0: const #20784u : u32
        let s_45_0: u32 = 20784;
        // D s_45_1: read-reg s_45_0:struct
        let s_45_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_45_0 as isize);
            tracer.read_register(s_45_0 as isize, value);
            value
        };
        // D s_45_2: call _get_SCTLR_EL2_Type_EnAS0(s_45_1)
        let s_45_2: bool = u_get_SCTLR_EL2_Type_EnAS0(state, tracer, s_45_1);
        // D s_45_3: cast zx s_45_2 -> bv
        let s_45_3: Bits = Bits::new(s_45_2 as u128, 1u16);
        // C s_45_4: const #0u : u8
        let s_45_4: bool = false;
        // C s_45_5: cast zx s_45_4 -> bv
        let s_45_5: Bits = Bits::new(s_45_4 as u128, 1u16);
        // D s_45_6: cmp-eq s_45_3 s_45_5
        let s_45_6: bool = ((s_45_3) == (s_45_5));
        // D s_45_7: write-var trap <= s_45_6
        fn_state.trap = s_45_6;
        // C s_45_8: const #432u : u32
        let s_45_8: u32 = 432;
        // D s_45_9: read-reg s_45_8:u8
        let s_45_9: u8 = {
            let value = state.read_register::<u8>(s_45_8 as isize);
            tracer.read_register(s_45_8 as isize, value);
            value
        };
        // D s_45_10: write-var target_el <= s_45_9
        fn_state.target_el = s_45_9;
        // N s_45_11: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_46_0: const #90272u : u32
        let s_46_0: u32 = 90272;
        // D s_46_1: read-reg s_46_0:struct
        let s_46_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_46_0 as isize);
            tracer.read_register(s_46_0 as isize, value);
            value
        };
        // D s_46_2: call _get_SCTLR_EL1_Type_EnAS0(s_46_1)
        let s_46_2: bool = u_get_SCTLR_EL1_Type_EnAS0(state, tracer, s_46_1);
        // D s_46_3: cast zx s_46_2 -> bv
        let s_46_3: Bits = Bits::new(s_46_2 as u128, 1u16);
        // C s_46_4: const #0u : u8
        let s_46_4: bool = false;
        // C s_46_5: cast zx s_46_4 -> bv
        let s_46_5: Bits = Bits::new(s_46_4 as u128, 1u16);
        // D s_46_6: cmp-eq s_46_3 s_46_5
        let s_46_6: bool = ((s_46_3) == (s_46_5));
        // D s_46_7: write-var trap <= s_46_6
        fn_state.trap = s_46_6;
        // C s_46_8: const #() : ()
        let s_46_8: () = ();
        // S s_46_9: call EL2Enabled(s_46_8)
        let s_46_9: bool = EL2Enabled(state, tracer, s_46_8);
        // N s_46_10: branch s_46_9 b52 b47
        if s_46_9 {
            return block_52(state, tracer, fn_state);
        } else {
            return block_47(state, tracer, fn_state);
        };
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_47_0: const #0u : u8
        let s_47_0: bool = false;
        // D s_47_1: write-var gs#24719 <= s_47_0
        fn_state.gs_24719 = s_47_0;
        // N s_47_2: jump b48
        return block_48(state, tracer, fn_state);
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_48_0: read-var gs#24719:u8
        let s_48_0: bool = fn_state.gs_24719;
        // N s_48_1: branch s_48_0 b51 b49
        if s_48_0 {
            return block_51(state, tracer, fn_state);
        } else {
            return block_49(state, tracer, fn_state);
        };
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_49_0: const #440u : u32
        let s_49_0: u32 = 440;
        // D s_49_1: read-reg s_49_0:u8
        let s_49_1: u8 = {
            let value = state.read_register::<u8>(s_49_0 as isize);
            tracer.read_register(s_49_0 as isize, value);
            value
        };
        // D s_49_2: write-var target_el <= s_49_1
        fn_state.target_el = s_49_1;
        // N s_49_3: jump b50
        return block_50(state, tracer, fn_state);
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_50_0: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_51_0: const #432u : u32
        let s_51_0: u32 = 432;
        // D s_51_1: read-reg s_51_0:u8
        let s_51_1: u8 = {
            let value = state.read_register::<u8>(s_51_0 as isize);
            tracer.read_register(s_51_0 as isize, value);
            value
        };
        // D s_51_2: write-var target_el <= s_51_1
        fn_state.target_el = s_51_1;
        // N s_51_3: jump b50
        return block_50(state, tracer, fn_state);
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_52_0: const #102552u : u32
        let s_52_0: u32 = 102552;
        // D s_52_1: read-reg s_52_0:struct
        let s_52_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_52_0 as isize);
            tracer.read_register(s_52_0 as isize, value);
            value
        };
        // D s_52_2: call _get_HCR_EL2_Type_TGE(s_52_1)
        let s_52_2: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_52_1);
        // D s_52_3: cast zx s_52_2 -> bv
        let s_52_3: Bits = Bits::new(s_52_2 as u128, 1u16);
        // C s_52_4: const #1u : u8
        let s_52_4: bool = true;
        // C s_52_5: cast zx s_52_4 -> bv
        let s_52_5: Bits = Bits::new(s_52_4 as u128, 1u16);
        // D s_52_6: cmp-eq s_52_3 s_52_5
        let s_52_6: bool = ((s_52_3) == (s_52_5));
        // D s_52_7: write-var gs#24719 <= s_52_6
        fn_state.gs_24719 = s_52_6;
        // N s_52_8: jump b48
        return block_48(state, tracer, fn_state);
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_53_0: panic
        panic!("{:?}", ());
        // N s_53_1: return
        return;
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_54_0: const #() : ()
        let s_54_0: () = ();
        // S s_54_1: call EL3SDDUndefPriority(s_54_0)
        let s_54_1: bool = EL3SDDUndefPriority(state, tracer, s_54_0);
        // D s_54_2: write-var gs#24702 <= s_54_1
        fn_state.gs_24702 = s_54_1;
        // N s_54_3: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_55_0: const #90704u : u32
        let s_55_0: u32 = 90704;
        // D s_55_1: read-reg s_55_0:struct
        let s_55_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_55_0 as isize);
            tracer.read_register(s_55_0 as isize, value);
            value
        };
        // D s_55_2: call _get_SCR_EL3_Type_EnAS0(s_55_1)
        let s_55_2: bool = u_get_SCR_EL3_Type_EnAS0(state, tracer, s_55_1);
        // D s_55_3: cast zx s_55_2 -> bv
        let s_55_3: Bits = Bits::new(s_55_2 as u128, 1u16);
        // C s_55_4: const #0u : u8
        let s_55_4: bool = false;
        // C s_55_5: cast zx s_55_4 -> bv
        let s_55_5: Bits = Bits::new(s_55_4 as u128, 1u16);
        // D s_55_6: cmp-eq s_55_3 s_55_5
        let s_55_6: bool = ((s_55_3) == (s_55_5));
        // D s_55_7: write-var gs#24701 <= s_55_6
        fn_state.gs_24701 = s_55_6;
        // N s_55_8: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_56_0: const #424u : u32
        let s_56_0: u32 = 424;
        // D s_56_1: read-reg s_56_0:u8
        let s_56_1: u8 = {
            let value = state.read_register::<u8>(s_56_0 as isize);
            tracer.read_register(s_56_0 as isize, value);
            value
        };
        // C s_56_2: const #2u : u8
        let s_56_2: u8 = 2;
        // D s_56_3: cmp-lt s_56_1 s_56_2
        let s_56_3: bool = ((s_56_1) < (s_56_2));
        // D s_56_4: write-var gs#24700 <= s_56_3
        fn_state.gs_24700 = s_56_3;
        // N s_56_5: jump b2
        return block_2(state, tracer, fn_state);
    }
}
