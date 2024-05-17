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
use u_get_HSTR_Type_T5::*;
use Mk_AMEVCNTR1_Type::*;
use IsHighestEL::*;
use R_read::*;
use AMEVCNTR1_set::*;
use ELUsingAArch32::*;
use u_get_HSTR_EL2_Type_T5::*;
use EL2Enabled::*;
use HSTR_read::*;
use IsG1ActivityMonitorImplemented::*;
use common::*;
pub fn AMEVCNTR1_SysRegWrite64_18488936f5130285<T: Tracer>(
    state: &mut State,
    tracer: &T,
    el: u8,
    coproc: u8,
    opc1: u8,
    CRm: u8,
    t: i128,
    t2: i128,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_104538: bool,
        gs_104540: bool,
        gs_104544: bool,
        gs_104541: bool,
        gs_104534: bool,
        gs_104535: bool,
        el: u8,
        coproc: u8,
        opc1: u8,
        CRm: u8,
        t: i128,
        t2: i128,
    }
    let fn_state = FunctionState {
        el,
        coproc,
        opc1,
        CRm,
        t,
        t2,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #104936u : u32
        let s_0_0: u32 = 104936;
        // D s_0_1: read-reg s_0_0:struct
        let s_0_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_0 as isize);
            tracer.read_register(s_0_0 as isize, value);
            value
        };
        // D s_0_2: call _get_HSTR_EL2_Type_T5(s_0_1)
        let s_0_2: bool = u_get_HSTR_EL2_Type_T5(state, tracer, s_0_1);
        // C s_0_3: const #() : ()
        let s_0_3: () = ();
        // S s_0_4: call HSTR_read(s_0_3)
        let s_0_4: ProductType700c18a878c5601b = HSTR_read(state, tracer, s_0_3);
        // S s_0_5: call _get_HSTR_Type_T5(s_0_4)
        let s_0_5: bool = u_get_HSTR_Type_T5(state, tracer, s_0_4);
        // C s_0_6: const #4s : i
        let s_0_6: i128 = 4;
        // S s_0_7: call IsG1ActivityMonitorImplemented(s_0_6)
        let s_0_7: bool = IsG1ActivityMonitorImplemented(state, tracer, s_0_6);
        // S s_0_8: not s_0_7
        let s_0_8: bool = !s_0_7;
        // N s_0_9: branch s_0_8 b28 b1
        if s_0_8 {
            return block_28(state, tracer, fn_state);
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
        // N s_1_7: branch s_1_6 b27 b2
        if s_1_6 {
            return block_27(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #0u : u8
        let s_2_0: bool = false;
        // D s_2_1: write-var gs#104534 <= s_2_0
        fn_state.gs_104534 = s_2_0;
        // N s_2_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var gs#104534:u8
        let s_3_0: bool = fn_state.gs_104534;
        // N s_3_1: branch s_3_0 b26 b4
        if s_3_0 {
            return block_26(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #0u : u8
        let s_4_0: bool = false;
        // D s_4_1: write-var gs#104535 <= s_4_0
        fn_state.gs_104535 = s_4_0;
        // N s_4_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var gs#104535:u8
        let s_5_0: bool = fn_state.gs_104535;
        // N s_5_1: branch s_5_0 b25 b6
        if s_5_0 {
            return block_25(state, tracer, fn_state);
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
        // D s_6_1: write-var gs#104538 <= s_6_0
        fn_state.gs_104538 = s_6_0;
        // N s_6_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var gs#104538:u8
        let s_7_0: bool = fn_state.gs_104538;
        // N s_7_1: branch s_7_0 b24 b8
        if s_7_0 {
            return block_24(state, tracer, fn_state);
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
        // C s_9_0: const #16975u : u32
        let s_9_0: u32 = 16975;
        // D s_9_1: read-reg s_9_0:u8
        let s_9_1: u8 = {
            let value = state.read_register::<u8>(s_9_0 as isize);
            tracer.read_register(s_9_0 as isize, value);
            value
        };
        // D s_9_2: cast zx s_9_1 -> bv
        let s_9_2: Bits = Bits::new(s_9_1 as u128, 2u16);
        // C s_9_3: const #440u : u32
        let s_9_3: u32 = 440;
        // D s_9_4: read-reg s_9_3:u8
        let s_9_4: u8 = {
            let value = state.read_register::<u8>(s_9_3 as isize);
            tracer.read_register(s_9_3 as isize, value);
            value
        };
        // D s_9_5: cast zx s_9_4 -> bv
        let s_9_5: Bits = Bits::new(s_9_4 as u128, 2u16);
        // D s_9_6: cmp-eq s_9_2 s_9_5
        let s_9_6: bool = ((s_9_2) == (s_9_5));
        // N s_9_7: branch s_9_6 b23 b10
        if s_9_6 {
            return block_23(state, tracer, fn_state);
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
        // D s_10_1: write-var gs#104540 <= s_10_0
        fn_state.gs_104540 = s_10_0;
        // N s_10_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var gs#104540:u8
        let s_11_0: bool = fn_state.gs_104540;
        // N s_11_1: branch s_11_0 b22 b12
        if s_11_0 {
            return block_22(state, tracer, fn_state);
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
        // D s_12_1: write-var gs#104541 <= s_12_0
        fn_state.gs_104541 = s_12_0;
        // N s_12_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var gs#104541:u8
        let s_13_0: bool = fn_state.gs_104541;
        // N s_13_1: branch s_13_0 b21 b14
        if s_13_0 {
            return block_21(state, tracer, fn_state);
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
        // D s_14_1: write-var gs#104544 <= s_14_0
        fn_state.gs_104544 = s_14_0;
        // N s_14_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var gs#104544:u8
        let s_15_0: bool = fn_state.gs_104544;
        // N s_15_1: branch s_15_0 b20 b16
        if s_15_0 {
            return block_20(state, tracer, fn_state);
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
        // C s_17_0: const #16975u : u32
        let s_17_0: u32 = 16975;
        // D s_17_1: read-reg s_17_0:u8
        let s_17_1: u8 = {
            let value = state.read_register::<u8>(s_17_0 as isize);
            tracer.read_register(s_17_0 as isize, value);
            value
        };
        // D s_17_2: call IsHighestEL(s_17_1)
        let s_17_2: bool = IsHighestEL(state, tracer, s_17_1);
        // N s_17_3: branch s_17_2 b19 b18
        if s_17_2 {
            return block_19(state, tracer, fn_state);
        } else {
            return block_18(state, tracer, fn_state);
        };
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_18_0: panic
        panic!("{:?}", ());
        // N s_18_1: return
        return;
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var t2:i
        let s_19_0: i128 = fn_state.t2;
        // D s_19_1: call R_read(s_19_0)
        let s_19_1: u32 = R_read(state, tracer, s_19_0);
        // D s_19_2: read-var t:i
        let s_19_2: i128 = fn_state.t;
        // D s_19_3: call R_read(s_19_2)
        let s_19_3: u32 = R_read(state, tracer, s_19_2);
        // D s_19_4: cast zx s_19_1 -> bv
        let s_19_4: Bits = Bits::new(s_19_1 as u128, 32u16);
        // D s_19_5: cast zx s_19_3 -> bv
        let s_19_5: Bits = Bits::new(s_19_3 as u128, 32u16);
        // D s_19_6: cast reint s_19_4 -> u128
        let s_19_6: u128 = (s_19_4.value() as u128);
        // D s_19_7: size-of s_19_4
        let s_19_7: u16 = s_19_4.length();
        // D s_19_8: cast reint s_19_5 -> u128
        let s_19_8: u128 = (s_19_5.value() as u128);
        // D s_19_9: size-of s_19_5
        let s_19_9: u16 = s_19_5.length();
        // D s_19_10: lsl s_19_6 s_19_9
        let s_19_10: u128 = s_19_6 << s_19_9;
        // D s_19_11: or s_19_10 s_19_8
        let s_19_11: u128 = ((s_19_10) | (s_19_8));
        // D s_19_12: add s_19_7 s_19_9
        let s_19_12: u16 = (s_19_7 + s_19_9);
        // D s_19_13: create-bits s_19_11 s_19_12
        let s_19_13: Bits = Bits::new(s_19_11, s_19_12);
        // D s_19_14: cast reint s_19_13 -> u64
        let s_19_14: u64 = (s_19_13.value() as u64);
        // D s_19_15: call Mk_AMEVCNTR1_Type(s_19_14)
        let s_19_15: ProductType5c790c8ef59cc8b2 = Mk_AMEVCNTR1_Type(
            state,
            tracer,
            s_19_14,
        );
        // C s_19_16: const #4s : i64
        let s_19_16: i64 = 4;
        // D s_19_17: call AMEVCNTR1_set(s_19_16, s_19_15)
        let s_19_17: () = AMEVCNTR1_set(state, tracer, s_19_16, s_19_15);
        // N s_19_18: return
        return;
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_20_0: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_21_0: const #0u : u8
        let s_21_0: bool = false;
        // D s_21_1: write-var gs#104544 <= s_21_0
        fn_state.gs_104544 = s_21_0;
        // N s_21_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_22_0: const #432u : u32
        let s_22_0: u32 = 432;
        // D s_22_1: read-reg s_22_0:u8
        let s_22_1: u8 = {
            let value = state.read_register::<u8>(s_22_0 as isize);
            tracer.read_register(s_22_0 as isize, value);
            value
        };
        // D s_22_2: call ELUsingAArch32(s_22_1)
        let s_22_2: bool = ELUsingAArch32(state, tracer, s_22_1);
        // D s_22_3: write-var gs#104541 <= s_22_2
        fn_state.gs_104541 = s_22_2;
        // N s_22_4: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_23_0: const #() : ()
        let s_23_0: () = ();
        // S s_23_1: call EL2Enabled(s_23_0)
        let s_23_1: bool = EL2Enabled(state, tracer, s_23_0);
        // D s_23_2: write-var gs#104540 <= s_23_1
        fn_state.gs_104540 = s_23_1;
        // N s_23_3: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_24_0: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_25_0: const #0u : u8
        let s_25_0: bool = false;
        // D s_25_1: write-var gs#104538 <= s_25_0
        fn_state.gs_104538 = s_25_0;
        // N s_25_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_26_0: const #432u : u32
        let s_26_0: u32 = 432;
        // D s_26_1: read-reg s_26_0:u8
        let s_26_1: u8 = {
            let value = state.read_register::<u8>(s_26_0 as isize);
            tracer.read_register(s_26_0 as isize, value);
            value
        };
        // D s_26_2: call ELUsingAArch32(s_26_1)
        let s_26_2: bool = ELUsingAArch32(state, tracer, s_26_1);
        // D s_26_3: not s_26_2
        let s_26_3: bool = !s_26_2;
        // D s_26_4: write-var gs#104535 <= s_26_3
        fn_state.gs_104535 = s_26_3;
        // N s_26_5: jump b5
        return block_5(state, tracer, fn_state);
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
        // D s_27_2: write-var gs#104534 <= s_27_1
        fn_state.gs_104534 = s_27_1;
        // N s_27_3: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_28_0: panic
        panic!("{:?}", ());
        // N s_28_1: return
        return;
    }
}
