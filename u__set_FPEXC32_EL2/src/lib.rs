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
use u_get_FPEXC32_EL2_Type_IDF::*;
use u_get_FPEXC32_EL2_Type_IOF::*;
use u__IMPDEF_boolean::*;
use u_get_FPEXC32_EL2_Type_UFF::*;
use u_get_FPEXC32_EL2_Type_IXF::*;
use u_get_FPEXC32_EL2_Type_OFF::*;
use u_get_FPEXC32_EL2_Type_DZF::*;
use common::*;
pub fn u__set_FPEXC32_EL2<T: Tracer>(
    state: &mut State,
    tracer: &T,
    value_name: ProductType5c790c8ef59cc8b2,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        r: ProductType5c790c8ef59cc8b2,
        value_name: ProductType5c790c8ef59cc8b2,
    }
    let fn_state = FunctionState {
        value_name,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var value_name:struct
        let s_0_0: ProductType5c790c8ef59cc8b2 = fn_state.value_name;
        // D s_0_1: write-var r <= s_0_0
        fn_state.r = s_0_0;
        // C s_0_2: const #21024u : u32
        let s_0_2: u32 = 21024;
        // D s_0_3: read-reg s_0_2:struct
        let s_0_3: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_2 as isize);
            tracer.read_register(s_0_2 as isize, value);
            value
        };
        // C s_0_4: const #21024u : u32
        let s_0_4: u32 = 21024;
        // N s_0_5: write-reg s_0_4 <= s_0_3
        let s_0_5: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_0_4 as isize, s_0_3);
            tracer.write_register(s_0_4 as isize, s_0_3);
        };
        // C s_0_6: const #21024u : u32
        let s_0_6: u32 = 21024;
        // D s_0_7: read-reg s_0_6:struct
        let s_0_7: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_6 as isize);
            tracer.read_register(s_0_6 as isize, value);
            value
        };
        // C s_0_8: const #21024u : u32
        let s_0_8: u32 = 21024;
        // N s_0_9: write-reg s_0_8 <= s_0_7
        let s_0_9: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_0_8 as isize, s_0_7);
            tracer.write_register(s_0_8 as isize, s_0_7);
        };
        // C s_0_10: const #21024u : u32
        let s_0_10: u32 = 21024;
        // D s_0_11: read-reg s_0_10:struct
        let s_0_11: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_10 as isize);
            tracer.read_register(s_0_10 as isize, value);
            value
        };
        // C s_0_12: const #21024u : u32
        let s_0_12: u32 = 21024;
        // N s_0_13: write-reg s_0_12 <= s_0_11
        let s_0_13: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_0_12 as isize, s_0_11);
            tracer.write_register(s_0_12 as isize, s_0_11);
        };
        // C s_0_14: const #"IMPLEMENTED_trapping of Input Denormal floating-point exceptions" : str
        let s_0_14: &'static str = "IMPLEMENTED_trapping of Input Denormal floating-point exceptions";
        // S s_0_15: call __IMPDEF_boolean(s_0_14)
        let s_0_15: bool = u__IMPDEF_boolean(state, tracer, s_0_14);
        // S s_0_16: not s_0_15
        let s_0_16: bool = !s_0_15;
        // S s_0_17: not s_0_16
        let s_0_17: bool = !s_0_16;
        // N s_0_18: branch s_0_17 b17 b1
        if s_0_17 {
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
        // N s_1_0: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #"IMPLEMENTED_trapping of Inexact floating-point exceptions" : str
        let s_2_0: &'static str = "IMPLEMENTED_trapping of Inexact floating-point exceptions";
        // S s_2_1: call __IMPDEF_boolean(s_2_0)
        let s_2_1: bool = u__IMPDEF_boolean(state, tracer, s_2_0);
        // S s_2_2: not s_2_1
        let s_2_2: bool = !s_2_1;
        // S s_2_3: not s_2_2
        let s_2_3: bool = !s_2_2;
        // N s_2_4: branch s_2_3 b16 b3
        if s_2_3 {
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
        // N s_3_0: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #"IMPLEMENTED_trapping of Underflow floating-point exceptions" : str
        let s_4_0: &'static str = "IMPLEMENTED_trapping of Underflow floating-point exceptions";
        // S s_4_1: call __IMPDEF_boolean(s_4_0)
        let s_4_1: bool = u__IMPDEF_boolean(state, tracer, s_4_0);
        // S s_4_2: not s_4_1
        let s_4_2: bool = !s_4_1;
        // S s_4_3: not s_4_2
        let s_4_3: bool = !s_4_2;
        // N s_4_4: branch s_4_3 b15 b5
        if s_4_3 {
            return block_15(state, tracer, fn_state);
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
        // C s_6_0: const #"IMPLEMENTED_trapping of Overflow floating-point exceptions" : str
        let s_6_0: &'static str = "IMPLEMENTED_trapping of Overflow floating-point exceptions";
        // S s_6_1: call __IMPDEF_boolean(s_6_0)
        let s_6_1: bool = u__IMPDEF_boolean(state, tracer, s_6_0);
        // S s_6_2: not s_6_1
        let s_6_2: bool = !s_6_1;
        // S s_6_3: not s_6_2
        let s_6_3: bool = !s_6_2;
        // N s_6_4: branch s_6_3 b14 b7
        if s_6_3 {
            return block_14(state, tracer, fn_state);
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
        // C s_8_0: const #"IMPLEMENTED_trapping of Divide by Zero floating-point exceptions" : str
        let s_8_0: &'static str = "IMPLEMENTED_trapping of Divide by Zero floating-point exceptions";
        // S s_8_1: call __IMPDEF_boolean(s_8_0)
        let s_8_1: bool = u__IMPDEF_boolean(state, tracer, s_8_0);
        // S s_8_2: not s_8_1
        let s_8_2: bool = !s_8_1;
        // S s_8_3: not s_8_2
        let s_8_3: bool = !s_8_2;
        // N s_8_4: branch s_8_3 b13 b9
        if s_8_3 {
            return block_13(state, tracer, fn_state);
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
        // C s_10_0: const #"IMPLEMENTED_trapping of Invalid Operation floating-point exceptions" : str
        let s_10_0: &'static str = "IMPLEMENTED_trapping of Invalid Operation floating-point exceptions";
        // S s_10_1: call __IMPDEF_boolean(s_10_0)
        let s_10_1: bool = u__IMPDEF_boolean(state, tracer, s_10_0);
        // S s_10_2: not s_10_1
        let s_10_2: bool = !s_10_1;
        // S s_10_3: not s_10_2
        let s_10_3: bool = !s_10_2;
        // N s_10_4: branch s_10_3 b12 b11
        if s_10_3 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_11_0: return
        return;
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var r:struct
        let s_12_0: ProductType5c790c8ef59cc8b2 = fn_state.r;
        // D s_12_1: call _get_FPEXC32_EL2_Type_IOF(s_12_0)
        let s_12_1: bool = u_get_FPEXC32_EL2_Type_IOF(state, tracer, s_12_0);
        // C s_12_2: const #21024u : u32
        let s_12_2: u32 = 21024;
        // D s_12_3: read-reg s_12_2:struct
        let s_12_3: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_12_2 as isize);
            tracer.read_register(s_12_2 as isize, value);
            value
        };
        // C s_12_4: const #21024u : u32
        let s_12_4: u32 = 21024;
        // N s_12_5: write-reg s_12_4 <= s_12_3
        let s_12_5: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_12_4 as isize, s_12_3);
            tracer.write_register(s_12_4 as isize, s_12_3);
        };
        // N s_12_6: return
        return;
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var r:struct
        let s_13_0: ProductType5c790c8ef59cc8b2 = fn_state.r;
        // D s_13_1: call _get_FPEXC32_EL2_Type_DZF(s_13_0)
        let s_13_1: bool = u_get_FPEXC32_EL2_Type_DZF(state, tracer, s_13_0);
        // C s_13_2: const #21024u : u32
        let s_13_2: u32 = 21024;
        // D s_13_3: read-reg s_13_2:struct
        let s_13_3: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_13_2 as isize);
            tracer.read_register(s_13_2 as isize, value);
            value
        };
        // C s_13_4: const #21024u : u32
        let s_13_4: u32 = 21024;
        // N s_13_5: write-reg s_13_4 <= s_13_3
        let s_13_5: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_13_4 as isize, s_13_3);
            tracer.write_register(s_13_4 as isize, s_13_3);
        };
        // N s_13_6: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var r:struct
        let s_14_0: ProductType5c790c8ef59cc8b2 = fn_state.r;
        // D s_14_1: call _get_FPEXC32_EL2_Type_OFF(s_14_0)
        let s_14_1: bool = u_get_FPEXC32_EL2_Type_OFF(state, tracer, s_14_0);
        // C s_14_2: const #21024u : u32
        let s_14_2: u32 = 21024;
        // D s_14_3: read-reg s_14_2:struct
        let s_14_3: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_14_2 as isize);
            tracer.read_register(s_14_2 as isize, value);
            value
        };
        // C s_14_4: const #21024u : u32
        let s_14_4: u32 = 21024;
        // N s_14_5: write-reg s_14_4 <= s_14_3
        let s_14_5: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_14_4 as isize, s_14_3);
            tracer.write_register(s_14_4 as isize, s_14_3);
        };
        // N s_14_6: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var r:struct
        let s_15_0: ProductType5c790c8ef59cc8b2 = fn_state.r;
        // D s_15_1: call _get_FPEXC32_EL2_Type_UFF(s_15_0)
        let s_15_1: bool = u_get_FPEXC32_EL2_Type_UFF(state, tracer, s_15_0);
        // C s_15_2: const #21024u : u32
        let s_15_2: u32 = 21024;
        // D s_15_3: read-reg s_15_2:struct
        let s_15_3: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_15_2 as isize);
            tracer.read_register(s_15_2 as isize, value);
            value
        };
        // C s_15_4: const #21024u : u32
        let s_15_4: u32 = 21024;
        // N s_15_5: write-reg s_15_4 <= s_15_3
        let s_15_5: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_15_4 as isize, s_15_3);
            tracer.write_register(s_15_4 as isize, s_15_3);
        };
        // N s_15_6: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var r:struct
        let s_16_0: ProductType5c790c8ef59cc8b2 = fn_state.r;
        // D s_16_1: call _get_FPEXC32_EL2_Type_IXF(s_16_0)
        let s_16_1: bool = u_get_FPEXC32_EL2_Type_IXF(state, tracer, s_16_0);
        // C s_16_2: const #21024u : u32
        let s_16_2: u32 = 21024;
        // D s_16_3: read-reg s_16_2:struct
        let s_16_3: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_16_2 as isize);
            tracer.read_register(s_16_2 as isize, value);
            value
        };
        // C s_16_4: const #21024u : u32
        let s_16_4: u32 = 21024;
        // N s_16_5: write-reg s_16_4 <= s_16_3
        let s_16_5: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_16_4 as isize, s_16_3);
            tracer.write_register(s_16_4 as isize, s_16_3);
        };
        // N s_16_6: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var r:struct
        let s_17_0: ProductType5c790c8ef59cc8b2 = fn_state.r;
        // D s_17_1: call _get_FPEXC32_EL2_Type_IDF(s_17_0)
        let s_17_1: bool = u_get_FPEXC32_EL2_Type_IDF(state, tracer, s_17_0);
        // C s_17_2: const #21024u : u32
        let s_17_2: u32 = 21024;
        // D s_17_3: read-reg s_17_2:struct
        let s_17_3: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_17_2 as isize);
            tracer.read_register(s_17_2 as isize, value);
            value
        };
        // C s_17_4: const #21024u : u32
        let s_17_4: u32 = 21024;
        // N s_17_5: write-reg s_17_4 <= s_17_3
        let s_17_5: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_17_4 as isize, s_17_3);
            tracer.write_register(s_17_4 as isize, s_17_3);
        };
        // N s_17_6: jump b2
        return block_2(state, tracer, fn_state);
    }
}
