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
use Bit::*;
use u__IMPDEF_boolean::*;
use u_get_SCTLR_Type_CP15BEN::*;
use u_get_SCTLR_Type_ITD::*;
use common::*;
pub fn u__set_SCTLR_S<T: Tracer>(
    state: &mut State,
    tracer: &T,
    value_name: ProductType700c18a878c5601b,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        r: ProductType700c18a878c5601b,
        value_name: ProductType700c18a878c5601b,
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
        let s_0_0: ProductType700c18a878c5601b = fn_state.value_name;
        // D s_0_1: write-var r <= s_0_0
        fn_state.r = s_0_0;
        // C s_0_2: const #16456u : u32
        let s_0_2: u32 = 16456;
        // D s_0_3: read-reg s_0_2:struct
        let s_0_3: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_0_2 as isize);
            tracer.read_register(s_0_2 as isize, value);
            value
        };
        // C s_0_4: const #16456u : u32
        let s_0_4: u32 = 16456;
        // N s_0_5: write-reg s_0_4 <= s_0_3
        let s_0_5: () = {
            state.write_register::<ProductType700c18a878c5601b>(s_0_4 as isize, s_0_3);
            tracer.write_register(s_0_4 as isize, s_0_3);
        };
        // D s_0_6: read-var r.0:struct
        let s_0_6: u32 = fn_state.r._0;
        // C s_0_7: const #6s : i
        let s_0_7: i128 = 6;
        // C s_0_8: const #1s : i
        let s_0_8: i128 = 1;
        // D s_0_9: cast zx s_0_6 -> bv
        let s_0_9: Bits = Bits::new(s_0_6 as u128, 32u16);
        // D s_0_10: bit-extract s_0_9 s_0_7 s_0_8
        let s_0_10: Bits = (Bits::new(
            ((s_0_9) >> (s_0_7)).value(),
            u16::try_from(s_0_8).unwrap(),
        ));
        // D s_0_11: cast reint s_0_10 -> u8
        let s_0_11: bool = ((s_0_10.value()) != 0);
        // D s_0_12: call Bit(s_0_11)
        let s_0_12: bool = Bit(state, tracer, s_0_11);
        // C s_0_13: const #16456u : u32
        let s_0_13: u32 = 16456;
        // D s_0_14: read-reg s_0_13:struct
        let s_0_14: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_0_13 as isize);
            tracer.read_register(s_0_13 as isize, value);
            value
        };
        // C s_0_15: const #16456u : u32
        let s_0_15: u32 = 16456;
        // N s_0_16: write-reg s_0_15 <= s_0_14
        let s_0_16: () = {
            state.write_register::<ProductType700c18a878c5601b>(s_0_15 as isize, s_0_14);
            tracer.write_register(s_0_15 as isize, s_0_14);
        };
        // C s_0_17: const #16456u : u32
        let s_0_17: u32 = 16456;
        // D s_0_18: read-reg s_0_17:struct
        let s_0_18: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_0_17 as isize);
            tracer.read_register(s_0_17 as isize, value);
            value
        };
        // C s_0_19: const #16456u : u32
        let s_0_19: u32 = 16456;
        // N s_0_20: write-reg s_0_19 <= s_0_18
        let s_0_20: () = {
            state.write_register::<ProductType700c18a878c5601b>(s_0_19 as isize, s_0_18);
            tracer.write_register(s_0_19 as isize, s_0_18);
        };
        // C s_0_21: const #"IMPLEMENTED_ITD" : str
        let s_0_21: &'static str = "IMPLEMENTED_ITD";
        // S s_0_22: call __IMPDEF_boolean(s_0_21)
        let s_0_22: bool = u__IMPDEF_boolean(state, tracer, s_0_21);
        // S s_0_23: not s_0_22
        let s_0_23: bool = !s_0_22;
        // S s_0_24: not s_0_23
        let s_0_24: bool = !s_0_23;
        // N s_0_25: branch s_0_24 b5 b1
        if s_0_24 {
            return block_5(state, tracer, fn_state);
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
        // C s_2_0: const #"IMPLEMENTED_CP15BEN" : str
        let s_2_0: &'static str = "IMPLEMENTED_CP15BEN";
        // S s_2_1: call __IMPDEF_boolean(s_2_0)
        let s_2_1: bool = u__IMPDEF_boolean(state, tracer, s_2_0);
        // S s_2_2: not s_2_1
        let s_2_2: bool = !s_2_1;
        // S s_2_3: not s_2_2
        let s_2_3: bool = !s_2_2;
        // N s_2_4: branch s_2_3 b4 b3
        if s_2_3 {
            return block_4(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_3_0: return
        return;
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var r:struct
        let s_4_0: ProductType700c18a878c5601b = fn_state.r;
        // D s_4_1: call _get_SCTLR_Type_CP15BEN(s_4_0)
        let s_4_1: bool = u_get_SCTLR_Type_CP15BEN(state, tracer, s_4_0);
        // C s_4_2: const #16456u : u32
        let s_4_2: u32 = 16456;
        // D s_4_3: read-reg s_4_2:struct
        let s_4_3: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_4_2 as isize);
            tracer.read_register(s_4_2 as isize, value);
            value
        };
        // C s_4_4: const #16456u : u32
        let s_4_4: u32 = 16456;
        // N s_4_5: write-reg s_4_4 <= s_4_3
        let s_4_5: () = {
            state.write_register::<ProductType700c18a878c5601b>(s_4_4 as isize, s_4_3);
            tracer.write_register(s_4_4 as isize, s_4_3);
        };
        // N s_4_6: return
        return;
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var r:struct
        let s_5_0: ProductType700c18a878c5601b = fn_state.r;
        // D s_5_1: call _get_SCTLR_Type_ITD(s_5_0)
        let s_5_1: bool = u_get_SCTLR_Type_ITD(state, tracer, s_5_0);
        // C s_5_2: const #16456u : u32
        let s_5_2: u32 = 16456;
        // D s_5_3: read-reg s_5_2:struct
        let s_5_3: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_5_2 as isize);
            tracer.read_register(s_5_2 as isize, value);
            value
        };
        // C s_5_4: const #16456u : u32
        let s_5_4: u32 = 16456;
        // N s_5_5: write-reg s_5_4 <= s_5_3
        let s_5_5: () = {
            state.write_register::<ProductType700c18a878c5601b>(s_5_4 as isize, s_5_3);
            tracer.write_register(s_5_4 as isize, s_5_3);
        };
        // N s_5_6: jump b2
        return block_2(state, tracer, fn_state);
    }
}
