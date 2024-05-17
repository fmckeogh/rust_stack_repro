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
use Havev8p4Debug::*;
use ExternalInvasiveDebugEnabled::*;
use ExternalSecureInvasiveDebugEnabled::*;
use ExternalRootInvasiveDebugEnabled::*;
use SecurityStateAtEL::*;
use u_get_EDSCR_Type_INTdis::*;
use EDSCR_read::*;
use ExternalRealmInvasiveDebugEnabled::*;
use common::*;
pub fn ExternalDebugInterruptsDisabled<T: Tracer>(
    state: &mut State,
    tracer: &T,
    target: u8,
) -> bool {
    #[derive(Default)]
    struct FunctionState {
        gs_24353: bool,
        gs_24349: bool,
        int_dis: bool,
        gs_24342: bool,
        ss: u32,
        gs_24358: bool,
        gs_24352: bool,
        gs_24344: bool,
        target: u8,
    }
    let fn_state = FunctionState {
        target,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_0_0: read-var target:u8
        let s_0_0: u8 = fn_state.target;
        // D s_0_1: call SecurityStateAtEL(s_0_0)
        let s_0_1: u32 = SecurityStateAtEL(state, tracer, s_0_0);
        // D s_0_2: write-var ss <= s_0_1
        fn_state.ss = s_0_1;
        // C s_0_3: const #() : ()
        let s_0_3: () = ();
        // S s_0_4: call Havev8p4Debug(s_0_3)
        let s_0_4: bool = Havev8p4Debug(state, tracer, s_0_3);
        // N s_0_5: branch s_0_4 b30 b1
        if s_0_4 {
            return block_30(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_1_0: read-var target:u8
        let s_1_0: u8 = fn_state.target;
        // D s_1_1: cast zx s_1_0 -> bv
        let s_1_1: Bits = Bits::new(s_1_0 as u128, 2u16);
        // C s_1_2: const #424u : u32
        let s_1_2: u32 = 424;
        // D s_1_3: read-reg s_1_2:u8
        let s_1_3: u8 = {
            let value = state.read_register::<u8>(s_1_2 as isize);
            tracer.read_register(s_1_2 as isize, value);
            value
        };
        // D s_1_4: cast zx s_1_3 -> bv
        let s_1_4: Bits = Bits::new(s_1_3 as u128, 2u16);
        // D s_1_5: cmp-eq s_1_1 s_1_4
        let s_1_5: bool = ((s_1_1) == (s_1_4));
        // D s_1_6: not s_1_5
        let s_1_6: bool = !s_1_5;
        // N s_1_7: branch s_1_6 b8 b2
        if s_1_6 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_2_0: const #() : ()
        let s_2_0: () = ();
        // S s_2_1: call EDSCR_read(s_2_0)
        let s_2_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_2_0);
        // S s_2_2: call _get_EDSCR_Type_INTdis(s_2_1)
        let s_2_2: u8 = u_get_EDSCR_Type_INTdis(state, tracer, s_2_1);
        // S s_2_3: cast zx s_2_2 -> bv
        let s_2_3: Bits = Bits::new(s_2_2 as u128, 2u16);
        // C s_2_4: const #3u : u8
        let s_2_4: u8 = 3;
        // C s_2_5: cast zx s_2_4 -> bv
        let s_2_5: Bits = Bits::new(s_2_4 as u128, 2u16);
        // S s_2_6: cmp-eq s_2_3 s_2_5
        let s_2_6: bool = ((s_2_3) == (s_2_5));
        // N s_2_7: branch s_2_6 b7 b3
        if s_2_6 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_3_0: const #0u : u8
        let s_3_0: bool = false;
        // D s_3_1: write-var gs#24342 <= s_3_0
        fn_state.gs_24342 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_4_0: read-var gs#24342:u8
        let s_4_0: bool = fn_state.gs_24342;
        // D s_4_1: write-var int_dis <= s_4_0
        fn_state.int_dis = s_4_0;
        // N s_4_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_5_0: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_6_0: read-var int_dis:u8
        let s_6_0: bool = fn_state.int_dis;
        // N s_6_1: return s_6_0
        return s_6_0;
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_7_0: const #() : ()
        let s_7_0: () = ();
        // S s_7_1: call ExternalSecureInvasiveDebugEnabled(s_7_0)
        let s_7_1: bool = ExternalSecureInvasiveDebugEnabled(state, tracer, s_7_0);
        // D s_7_2: write-var gs#24342 <= s_7_1
        fn_state.gs_24342 = s_7_1;
        // N s_7_3: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_8_0: read-var target:u8
        let s_8_0: u8 = fn_state.target;
        // D s_8_1: cast zx s_8_0 -> bv
        let s_8_1: Bits = Bits::new(s_8_0 as u128, 2u16);
        // C s_8_2: const #432u : u32
        let s_8_2: u32 = 432;
        // D s_8_3: read-reg s_8_2:u8
        let s_8_3: u8 = {
            let value = state.read_register::<u8>(s_8_2 as isize);
            tracer.read_register(s_8_2 as isize, value);
            value
        };
        // D s_8_4: cast zx s_8_3 -> bv
        let s_8_4: Bits = Bits::new(s_8_3 as u128, 2u16);
        // D s_8_5: cmp-eq s_8_1 s_8_4
        let s_8_5: bool = ((s_8_1) == (s_8_4));
        // D s_8_6: not s_8_5
        let s_8_6: bool = !s_8_5;
        // N s_8_7: branch s_8_6 b16 b9
        if s_8_6 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_9_0: const #() : ()
        let s_9_0: () = ();
        // S s_9_1: call EDSCR_read(s_9_0)
        let s_9_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_9_0);
        // S s_9_2: call _get_EDSCR_Type_INTdis(s_9_1)
        let s_9_2: u8 = u_get_EDSCR_Type_INTdis(state, tracer, s_9_1);
        // C s_9_3: const #1s : i
        let s_9_3: i128 = 1;
        // S s_9_4: cast zx s_9_2 -> bv
        let s_9_4: Bits = Bits::new(s_9_2 as u128, 2u16);
        // C s_9_5: const #1s : i64
        let s_9_5: i64 = 1;
        // C s_9_6: cast zx s_9_5 -> i
        let s_9_6: i128 = (i128::try_from(s_9_5).unwrap());
        // C s_9_7: const #0s : i
        let s_9_7: i128 = 0;
        // C s_9_8: add s_9_7 s_9_6
        let s_9_8: i128 = (s_9_7 + s_9_6);
        // D s_9_9: bit-extract s_9_4 s_9_3 s_9_8
        let s_9_9: Bits = (Bits::new(
            ((s_9_4) >> (s_9_3)).value(),
            u16::try_from(s_9_8).unwrap(),
        ));
        // D s_9_10: cast reint s_9_9 -> u8
        let s_9_10: bool = ((s_9_9.value()) != 0);
        // D s_9_11: cast zx s_9_10 -> bv
        let s_9_11: Bits = Bits::new(s_9_10 as u128, 1u16);
        // C s_9_12: const #1u : u8
        let s_9_12: bool = true;
        // C s_9_13: cast zx s_9_12 -> bv
        let s_9_13: Bits = Bits::new(s_9_12 as u128, 1u16);
        // D s_9_14: cmp-eq s_9_11 s_9_13
        let s_9_14: bool = ((s_9_11) == (s_9_13));
        // D s_9_15: not s_9_14
        let s_9_15: bool = !s_9_14;
        // N s_9_16: branch s_9_15 b15 b10
        if s_9_15 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_10_0: const #1u : u8
        let s_10_0: bool = true;
        // D s_10_1: write-var gs#24344 <= s_10_0
        fn_state.gs_24344 = s_10_0;
        // N s_10_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_11_0: read-var gs#24344:u8
        let s_11_0: bool = fn_state.gs_24344;
        // N s_11_1: branch s_11_0 b14 b12
        if s_11_0 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_12_0: const #0u : u8
        let s_12_0: bool = false;
        // D s_12_1: write-var gs#24349 <= s_12_0
        fn_state.gs_24349 = s_12_0;
        // N s_12_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_13_0: read-var gs#24349:u8
        let s_13_0: bool = fn_state.gs_24349;
        // D s_13_1: write-var int_dis <= s_13_0
        fn_state.int_dis = s_13_0;
        // N s_13_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_14_0: const #() : ()
        let s_14_0: () = ();
        // S s_14_1: call ExternalInvasiveDebugEnabled(s_14_0)
        let s_14_1: bool = ExternalInvasiveDebugEnabled(state, tracer, s_14_0);
        // D s_14_2: write-var gs#24349 <= s_14_1
        fn_state.gs_24349 = s_14_1;
        // N s_14_3: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_15_0: const #0u : u8
        let s_15_0: bool = false;
        // D s_15_1: write-var gs#24344 <= s_15_0
        fn_state.gs_24344 = s_15_0;
        // N s_15_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_16_0: read-var target:u8
        let s_16_0: u8 = fn_state.target;
        // D s_16_1: cast zx s_16_0 -> bv
        let s_16_1: Bits = Bits::new(s_16_0 as u128, 2u16);
        // C s_16_2: const #440u : u32
        let s_16_2: u32 = 440;
        // D s_16_3: read-reg s_16_2:u8
        let s_16_3: u8 = {
            let value = state.read_register::<u8>(s_16_2 as isize);
            tracer.read_register(s_16_2 as isize, value);
            value
        };
        // D s_16_4: cast zx s_16_3 -> bv
        let s_16_4: Bits = Bits::new(s_16_3 as u128, 2u16);
        // D s_16_5: cmp-eq s_16_1 s_16_4
        let s_16_5: bool = ((s_16_1) == (s_16_4));
        // D s_16_6: not s_16_5
        let s_16_6: bool = !s_16_5;
        // N s_16_7: branch s_16_6 b29 b17
        if s_16_6 {
            return block_29(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_17_0: read-var ss:u32
        let s_17_0: u32 = fn_state.ss;
        // C s_17_1: const #3u : u32
        let s_17_1: u32 = 3;
        // D s_17_2: cmp-eq s_17_0 s_17_1
        let s_17_2: bool = ((s_17_0) == (s_17_1));
        // N s_17_3: branch s_17_2 b22 b18
        if s_17_2 {
            return block_22(state, tracer, fn_state);
        } else {
            return block_18(state, tracer, fn_state);
        };
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_18_0: const #() : ()
        let s_18_0: () = ();
        // S s_18_1: call EDSCR_read(s_18_0)
        let s_18_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_18_0);
        // S s_18_2: call _get_EDSCR_Type_INTdis(s_18_1)
        let s_18_2: u8 = u_get_EDSCR_Type_INTdis(state, tracer, s_18_1);
        // S s_18_3: cast zx s_18_2 -> bv
        let s_18_3: Bits = Bits::new(s_18_2 as u128, 2u16);
        // C s_18_4: const #0u : u8
        let s_18_4: u8 = 0;
        // C s_18_5: cast zx s_18_4 -> bv
        let s_18_5: Bits = Bits::new(s_18_4 as u128, 2u16);
        // S s_18_6: cmp-ne s_18_3 s_18_5
        let s_18_6: bool = ((s_18_3) != (s_18_5));
        // N s_18_7: branch s_18_6 b21 b19
        if s_18_6 {
            return block_21(state, tracer, fn_state);
        } else {
            return block_19(state, tracer, fn_state);
        };
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_19_0: const #0u : u8
        let s_19_0: bool = false;
        // D s_19_1: write-var gs#24352 <= s_19_0
        fn_state.gs_24352 = s_19_0;
        // N s_19_2: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_20_0: read-var gs#24352:u8
        let s_20_0: bool = fn_state.gs_24352;
        // D s_20_1: write-var int_dis <= s_20_0
        fn_state.int_dis = s_20_0;
        // N s_20_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_21_0: const #() : ()
        let s_21_0: () = ();
        // S s_21_1: call ExternalInvasiveDebugEnabled(s_21_0)
        let s_21_1: bool = ExternalInvasiveDebugEnabled(state, tracer, s_21_0);
        // D s_21_2: write-var gs#24352 <= s_21_1
        fn_state.gs_24352 = s_21_1;
        // N s_21_3: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_22_0: const #() : ()
        let s_22_0: () = ();
        // S s_22_1: call EDSCR_read(s_22_0)
        let s_22_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_22_0);
        // S s_22_2: call _get_EDSCR_Type_INTdis(s_22_1)
        let s_22_2: u8 = u_get_EDSCR_Type_INTdis(state, tracer, s_22_1);
        // C s_22_3: const #1s : i
        let s_22_3: i128 = 1;
        // S s_22_4: cast zx s_22_2 -> bv
        let s_22_4: Bits = Bits::new(s_22_2 as u128, 2u16);
        // C s_22_5: const #1s : i64
        let s_22_5: i64 = 1;
        // C s_22_6: cast zx s_22_5 -> i
        let s_22_6: i128 = (i128::try_from(s_22_5).unwrap());
        // C s_22_7: const #0s : i
        let s_22_7: i128 = 0;
        // C s_22_8: add s_22_7 s_22_6
        let s_22_8: i128 = (s_22_7 + s_22_6);
        // D s_22_9: bit-extract s_22_4 s_22_3 s_22_8
        let s_22_9: Bits = (Bits::new(
            ((s_22_4) >> (s_22_3)).value(),
            u16::try_from(s_22_8).unwrap(),
        ));
        // D s_22_10: cast reint s_22_9 -> u8
        let s_22_10: bool = ((s_22_9.value()) != 0);
        // D s_22_11: cast zx s_22_10 -> bv
        let s_22_11: Bits = Bits::new(s_22_10 as u128, 1u16);
        // C s_22_12: const #1u : u8
        let s_22_12: bool = true;
        // C s_22_13: cast zx s_22_12 -> bv
        let s_22_13: Bits = Bits::new(s_22_12 as u128, 1u16);
        // D s_22_14: cmp-eq s_22_11 s_22_13
        let s_22_14: bool = ((s_22_11) == (s_22_13));
        // D s_22_15: not s_22_14
        let s_22_15: bool = !s_22_14;
        // N s_22_16: branch s_22_15 b28 b23
        if s_22_15 {
            return block_28(state, tracer, fn_state);
        } else {
            return block_23(state, tracer, fn_state);
        };
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_23_0: const #1u : u8
        let s_23_0: bool = true;
        // D s_23_1: write-var gs#24353 <= s_23_0
        fn_state.gs_24353 = s_23_0;
        // N s_23_2: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_24_0: read-var gs#24353:u8
        let s_24_0: bool = fn_state.gs_24353;
        // N s_24_1: branch s_24_0 b27 b25
        if s_24_0 {
            return block_27(state, tracer, fn_state);
        } else {
            return block_25(state, tracer, fn_state);
        };
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_25_0: const #0u : u8
        let s_25_0: bool = false;
        // D s_25_1: write-var gs#24358 <= s_25_0
        fn_state.gs_24358 = s_25_0;
        // N s_25_2: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_26_0: read-var gs#24358:u8
        let s_26_0: bool = fn_state.gs_24358;
        // D s_26_1: write-var int_dis <= s_26_0
        fn_state.int_dis = s_26_0;
        // N s_26_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_27_0: const #() : ()
        let s_27_0: () = ();
        // S s_27_1: call ExternalSecureInvasiveDebugEnabled(s_27_0)
        let s_27_1: bool = ExternalSecureInvasiveDebugEnabled(state, tracer, s_27_0);
        // D s_27_2: write-var gs#24358 <= s_27_1
        fn_state.gs_24358 = s_27_1;
        // N s_27_3: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_28_0: const #0u : u8
        let s_28_0: bool = false;
        // D s_28_1: write-var gs#24353 <= s_28_0
        fn_state.gs_24353 = s_28_0;
        // N s_28_2: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_29_0: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_30_0: const #() : ()
        let s_30_0: () = ();
        // S s_30_1: call EDSCR_read(s_30_0)
        let s_30_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_30_0);
        // S s_30_2: call _get_EDSCR_Type_INTdis(s_30_1)
        let s_30_2: u8 = u_get_EDSCR_Type_INTdis(state, tracer, s_30_1);
        // C s_30_3: const #0s : i
        let s_30_3: i128 = 0;
        // S s_30_4: cast zx s_30_2 -> bv
        let s_30_4: Bits = Bits::new(s_30_2 as u128, 2u16);
        // C s_30_5: const #1u : u64
        let s_30_5: u64 = 1;
        // D s_30_6: bit-extract s_30_4 s_30_3 s_30_5
        let s_30_6: Bits = (Bits::new(
            ((s_30_4) >> (s_30_3)).value(),
            u16::try_from(s_30_5).unwrap(),
        ));
        // D s_30_7: cast reint s_30_6 -> u8
        let s_30_7: bool = ((s_30_6.value()) != 0);
        // C s_30_8: const #0s : i
        let s_30_8: i128 = 0;
        // C s_30_9: const #0u : u64
        let s_30_9: u64 = 0;
        // D s_30_10: cast zx s_30_7 -> u64
        let s_30_10: u64 = (s_30_7 as u64);
        // C s_30_11: const #1u : u64
        let s_30_11: u64 = 1;
        // D s_30_12: and s_30_10 s_30_11
        let s_30_12: u64 = ((s_30_10) & (s_30_11));
        // D s_30_13: cmp-eq s_30_12 s_30_11
        let s_30_13: bool = ((s_30_12) == (s_30_11));
        // D s_30_14: lsl s_30_10 s_30_8
        let s_30_14: u64 = s_30_10 << s_30_8;
        // D s_30_15: or s_30_9 s_30_14
        let s_30_15: u64 = ((s_30_9) | (s_30_14));
        // D s_30_16: cmpl s_30_14
        let s_30_16: u64 = !s_30_14;
        // D s_30_17: and s_30_9 s_30_16
        let s_30_17: u64 = ((s_30_9) & (s_30_16));
        // D s_30_18: select s_30_13 s_30_15 s_30_17
        let s_30_18: u64 = if s_30_13 { s_30_15 } else { s_30_17 };
        // D s_30_19: cast trunc s_30_18 -> u8
        let s_30_19: bool = ((s_30_18) != 0);
        // D s_30_20: cast zx s_30_19 -> bv
        let s_30_20: Bits = Bits::new(s_30_19 as u128, 1u16);
        // C s_30_21: const #1u : u8
        let s_30_21: bool = true;
        // C s_30_22: cast zx s_30_21 -> bv
        let s_30_22: Bits = Bits::new(s_30_21 as u128, 1u16);
        // D s_30_23: cmp-eq s_30_20 s_30_22
        let s_30_23: bool = ((s_30_20) == (s_30_22));
        // N s_30_24: branch s_30_23 b32 b31
        if s_30_23 {
            return block_32(state, tracer, fn_state);
        } else {
            return block_31(state, tracer, fn_state);
        };
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_31_0: const #0u : u8
        let s_31_0: bool = false;
        // D s_31_1: write-var int_dis <= s_31_0
        fn_state.int_dis = s_31_0;
        // N s_31_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_32_0: const #0u : u32
        let s_32_0: u32 = 0;
        // D s_32_1: read-var ss:u32
        let s_32_1: u32 = fn_state.ss;
        // D s_32_2: cmp-eq s_32_0 s_32_1
        let s_32_2: bool = ((s_32_0) == (s_32_1));
        // D s_32_3: not s_32_2
        let s_32_3: bool = !s_32_2;
        // N s_32_4: branch s_32_3 b35 b33
        if s_32_3 {
            return block_35(state, tracer, fn_state);
        } else {
            return block_33(state, tracer, fn_state);
        };
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_33_0: const #() : ()
        let s_33_0: () = ();
        // S s_33_1: call ExternalInvasiveDebugEnabled(s_33_0)
        let s_33_1: bool = ExternalInvasiveDebugEnabled(state, tracer, s_33_0);
        // D s_33_2: write-var int_dis <= s_33_1
        fn_state.int_dis = s_33_1;
        // N s_33_3: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_34_0: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_35_0: const #3u : u32
        let s_35_0: u32 = 3;
        // D s_35_1: read-var ss:u32
        let s_35_1: u32 = fn_state.ss;
        // D s_35_2: cmp-eq s_35_0 s_35_1
        let s_35_2: bool = ((s_35_0) == (s_35_1));
        // D s_35_3: not s_35_2
        let s_35_3: bool = !s_35_2;
        // N s_35_4: branch s_35_3 b37 b36
        if s_35_3 {
            return block_37(state, tracer, fn_state);
        } else {
            return block_36(state, tracer, fn_state);
        };
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_36_0: const #() : ()
        let s_36_0: () = ();
        // S s_36_1: call ExternalSecureInvasiveDebugEnabled(s_36_0)
        let s_36_1: bool = ExternalSecureInvasiveDebugEnabled(state, tracer, s_36_0);
        // D s_36_2: write-var int_dis <= s_36_1
        fn_state.int_dis = s_36_1;
        // N s_36_3: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_37_0: const #2u : u32
        let s_37_0: u32 = 2;
        // D s_37_1: read-var ss:u32
        let s_37_1: u32 = fn_state.ss;
        // D s_37_2: cmp-eq s_37_0 s_37_1
        let s_37_2: bool = ((s_37_0) == (s_37_1));
        // D s_37_3: not s_37_2
        let s_37_3: bool = !s_37_2;
        // N s_37_4: branch s_37_3 b39 b38
        if s_37_3 {
            return block_39(state, tracer, fn_state);
        } else {
            return block_38(state, tracer, fn_state);
        };
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_38_0: const #() : ()
        let s_38_0: () = ();
        // S s_38_1: call ExternalRealmInvasiveDebugEnabled(s_38_0)
        let s_38_1: bool = ExternalRealmInvasiveDebugEnabled(state, tracer, s_38_0);
        // D s_38_2: write-var int_dis <= s_38_1
        fn_state.int_dis = s_38_1;
        // N s_38_3: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_39_0: const #1u : u32
        let s_39_0: u32 = 1;
        // D s_39_1: read-var ss:u32
        let s_39_1: u32 = fn_state.ss;
        // D s_39_2: cmp-eq s_39_0 s_39_1
        let s_39_2: bool = ((s_39_0) == (s_39_1));
        // D s_39_3: not s_39_2
        let s_39_3: bool = !s_39_2;
        // N s_39_4: branch s_39_3 b41 b40
        if s_39_3 {
            return block_41(state, tracer, fn_state);
        } else {
            return block_40(state, tracer, fn_state);
        };
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_40_0: const #() : ()
        let s_40_0: () = ();
        // S s_40_1: call ExternalRootInvasiveDebugEnabled(s_40_0)
        let s_40_1: bool = ExternalRootInvasiveDebugEnabled(state, tracer, s_40_0);
        // D s_40_2: write-var int_dis <= s_40_1
        fn_state.int_dis = s_40_1;
        // N s_40_3: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_41_0: jump b34
        return block_34(state, tracer, fn_state);
    }
}
