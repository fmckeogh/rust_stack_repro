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
use GenMPAMcurEL::*;
use SecurityStateAtEL::*;
use common::*;
pub fn NewAccDesc<T: Tracer>(
    state: &mut State,
    tracer: &T,
    acctype: u32,
) -> ProductType9878976b5bcce9c9 {
    #[derive(Default)]
    struct FunctionState {
        accdesc: ProductType9878976b5bcce9c9,
        acctype: u32,
    }
    let fn_state = FunctionState {
        acctype,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType9878976b5bcce9c9 {
        // D s_0_0: read-var acctype:u32
        let s_0_0: u32 = fn_state.acctype;
        // D s_0_1: write-var accdesc.1 <= s_0_0
        fn_state.accdesc._1 = s_0_0;
        // C s_0_2: const #16975u : u32
        let s_0_2: u32 = 16975;
        // D s_0_3: read-reg s_0_2:u8
        let s_0_3: u8 = {
            let value = state.read_register::<u8>(s_0_2 as isize);
            tracer.read_register(s_0_2 as isize, value);
            value
        };
        // D s_0_4: write-var accdesc.8 <= s_0_3
        fn_state.accdesc._8 = s_0_3;
        // C s_0_5: const #16975u : u32
        let s_0_5: u32 = 16975;
        // D s_0_6: read-reg s_0_5:u8
        let s_0_6: u8 = {
            let value = state.read_register::<u8>(s_0_5 as isize);
            tracer.read_register(s_0_5 as isize, value);
            value
        };
        // D s_0_7: call SecurityStateAtEL(s_0_6)
        let s_0_7: u32 = SecurityStateAtEL(state, tracer, s_0_6);
        // D s_0_8: write-var accdesc.25 <= s_0_7
        fn_state.accdesc._25 = s_0_7;
        // C s_0_9: const #0u : u8
        let s_0_9: bool = false;
        // D s_0_10: write-var accdesc.3 <= s_0_9
        fn_state.accdesc._3 = s_0_9;
        // C s_0_11: const #0u : u8
        let s_0_11: bool = false;
        // D s_0_12: write-var accdesc.2 <= s_0_11
        fn_state.accdesc._2 = s_0_11;
        // C s_0_13: const #0u : u8
        let s_0_13: bool = false;
        // D s_0_14: write-var accdesc.24 <= s_0_13
        fn_state.accdesc._24 = s_0_13;
        // C s_0_15: const #0u : u8
        let s_0_15: bool = false;
        // D s_0_16: write-var accdesc.12 <= s_0_15
        fn_state.accdesc._12 = s_0_15;
        // C s_0_17: const #0u : u8
        let s_0_17: bool = false;
        // D s_0_18: write-var accdesc.9 <= s_0_17
        fn_state.accdesc._9 = s_0_17;
        // C s_0_19: const #0u : u8
        let s_0_19: bool = false;
        // D s_0_20: write-var accdesc.21 <= s_0_19
        fn_state.accdesc._21 = s_0_19;
        // C s_0_21: const #0u : u8
        let s_0_21: bool = false;
        // D s_0_22: write-var accdesc.22 <= s_0_21
        fn_state.accdesc._22 = s_0_21;
        // C s_0_23: const #0u : u8
        let s_0_23: bool = false;
        // D s_0_24: write-var accdesc.4 <= s_0_23
        fn_state.accdesc._4 = s_0_23;
        // C s_0_25: const #0u : u8
        let s_0_25: bool = false;
        // D s_0_26: write-var accdesc.18 <= s_0_25
        fn_state.accdesc._18 = s_0_25;
        // C s_0_27: const #0u : u8
        let s_0_27: bool = false;
        // D s_0_28: write-var accdesc.23 <= s_0_27
        fn_state.accdesc._23 = s_0_27;
        // C s_0_29: const #0u : u8
        let s_0_29: bool = false;
        // D s_0_30: write-var accdesc.32 <= s_0_29
        fn_state.accdesc._32 = s_0_29;
        // C s_0_31: const #0u : u8
        let s_0_31: bool = false;
        // D s_0_32: write-var accdesc.20 <= s_0_31
        fn_state.accdesc._20 = s_0_31;
        // C s_0_33: const #0u : u8
        let s_0_33: bool = false;
        // D s_0_34: write-var accdesc.17 <= s_0_33
        fn_state.accdesc._17 = s_0_33;
        // C s_0_35: const #0u : u8
        let s_0_35: bool = false;
        // D s_0_36: write-var accdesc.11 <= s_0_35
        fn_state.accdesc._11 = s_0_35;
        // C s_0_37: const #0u : u8
        let s_0_37: bool = false;
        // D s_0_38: write-var accdesc.10 <= s_0_37
        fn_state.accdesc._10 = s_0_37;
        // C s_0_39: const #0u : u8
        let s_0_39: bool = false;
        // D s_0_40: write-var accdesc.7 <= s_0_39
        fn_state.accdesc._7 = s_0_39;
        // C s_0_41: const #0u : u8
        let s_0_41: bool = false;
        // D s_0_42: write-var accdesc.26 <= s_0_41
        fn_state.accdesc._26 = s_0_41;
        // C s_0_43: const #0u : u8
        let s_0_43: bool = false;
        // D s_0_44: write-var accdesc.13 <= s_0_43
        fn_state.accdesc._13 = s_0_43;
        // C s_0_45: const #0u : u8
        let s_0_45: bool = false;
        // D s_0_46: write-var accdesc.15 <= s_0_45
        fn_state.accdesc._15 = s_0_45;
        // C s_0_47: const #0u : u8
        let s_0_47: bool = false;
        // D s_0_48: write-var accdesc.0 <= s_0_47
        fn_state.accdesc._0 = s_0_47;
        // C s_0_49: const #0u : u8
        let s_0_49: bool = false;
        // D s_0_50: write-var accdesc.28 <= s_0_49
        fn_state.accdesc._28 = s_0_49;
        // C s_0_51: const #0u : u8
        let s_0_51: bool = false;
        // D s_0_52: write-var accdesc.27 <= s_0_51
        fn_state.accdesc._27 = s_0_51;
        // C s_0_53: const #0u : u8
        let s_0_53: bool = false;
        // D s_0_54: write-var accdesc.30 <= s_0_53
        fn_state.accdesc._30 = s_0_53;
        // D s_0_55: read-var acctype:u32
        let s_0_55: u32 = fn_state.acctype;
        // D s_0_56: call GenMPAMcurEL(s_0_55)
        let s_0_56: ProductTypee79b4310dbe79c8c = GenMPAMcurEL(state, tracer, s_0_55);
        // D s_0_57: write-var accdesc.16 <= s_0_56
        fn_state.accdesc._16 = s_0_56;
        // D s_0_58: read-var accdesc:struct
        let s_0_58: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // N s_0_59: return s_0_58
        return s_0_58;
    }
}
