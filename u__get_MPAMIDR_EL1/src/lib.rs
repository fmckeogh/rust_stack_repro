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
use Mk_MPAMIDR_EL1_Type::*;
use u_get_MPAMIDR_EL1_Type_HAS_HCR::*;
use common::*;
pub fn u__get_MPAMIDR_EL1<T: Tracer>(
    state: &mut State,
    tracer: &T,
    value_name: ProductType5c790c8ef59cc8b2,
) -> ProductType5c790c8ef59cc8b2 {
    #[derive(Default)]
    struct FunctionState {
        tmp: ProductType5c790c8ef59cc8b2,
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
    ) -> ProductType5c790c8ef59cc8b2 {
        // D s_0_0: read-var value_name:struct
        let s_0_0: ProductType5c790c8ef59cc8b2 = fn_state.value_name;
        // D s_0_1: write-var tmp <= s_0_0
        fn_state.tmp = s_0_0;
        // D s_0_2: read-var tmp.0:struct
        let s_0_2: u64 = fn_state.tmp._0;
        // C s_0_3: const #13979172148139327488u : u64
        let s_0_3: u64 = 13979172148139327488;
        // C s_0_4: cast zx s_0_3 -> bv
        let s_0_4: Bits = Bits::new(s_0_3 as u128, 64u16);
        // C s_0_5: not s_0_4
        let s_0_5: Bits = !s_0_4;
        // C s_0_6: cast reint s_0_5 -> u64
        let s_0_6: u64 = (s_0_5.value() as u64);
        // D s_0_7: cast zx s_0_2 -> bv
        let s_0_7: Bits = Bits::new(s_0_2 as u128, 64u16);
        // C s_0_8: cast zx s_0_6 -> bv
        let s_0_8: Bits = Bits::new(s_0_6 as u128, 64u16);
        // D s_0_9: and s_0_7 s_0_8
        let s_0_9: Bits = ((s_0_7) & (s_0_8));
        // D s_0_10: cast reint s_0_9 -> u64
        let s_0_10: u64 = (s_0_9.value() as u64);
        // D s_0_11: call Mk_MPAMIDR_EL1_Type(s_0_10)
        let s_0_11: ProductType5c790c8ef59cc8b2 = Mk_MPAMIDR_EL1_Type(
            state,
            tracer,
            s_0_10,
        );
        // D s_0_12: write-var tmp <= s_0_11
        fn_state.tmp = s_0_11;
        // C s_0_13: const #11032u : u32
        let s_0_13: u32 = 11032;
        // D s_0_14: read-reg s_0_13:struct
        let s_0_14: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_13 as isize);
            tracer.read_register(s_0_13 as isize, value);
            value
        };
        // D s_0_15: call _get_MPAMIDR_EL1_Type_HAS_HCR(s_0_14)
        let s_0_15: bool = u_get_MPAMIDR_EL1_Type_HAS_HCR(state, tracer, s_0_14);
        // D s_0_16: cast zx s_0_15 -> bv
        let s_0_16: Bits = Bits::new(s_0_15 as u128, 1u16);
        // C s_0_17: const #1u : u8
        let s_0_17: bool = true;
        // C s_0_18: cast zx s_0_17 -> bv
        let s_0_18: Bits = Bits::new(s_0_17 as u128, 1u16);
        // D s_0_19: cmp-eq s_0_16 s_0_18
        let s_0_19: bool = ((s_0_16) == (s_0_18));
        // D s_0_20: not s_0_19
        let s_0_20: bool = !s_0_19;
        // N s_0_21: branch s_0_20 b3 b1
        if s_0_20 {
            return block_3(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // N s_1_0: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // D s_2_0: read-var tmp:struct
        let s_2_0: ProductType5c790c8ef59cc8b2 = fn_state.tmp;
        // N s_2_1: return s_2_0
        return s_2_0;
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // D s_3_0: read-var tmp.0:struct
        let s_3_0: u64 = fn_state.tmp._0;
        // C s_3_1: const #64s : i
        let s_3_1: i128 = 64;
        // C s_3_2: const #1835008u : u24
        let s_3_2: u32 = 1835008;
        // C s_3_3: cast zx s_3_2 -> bv
        let s_3_3: Bits = Bits::new(s_3_2 as u128, 24u16);
        // D s_3_4: bits-cast zx s_3_3 -> bv length s_3_1
        let s_3_4: Bits = s_3_3.zero_extend(s_3_1);
        // D s_3_5: cast reint s_3_4 -> u64
        let s_3_5: u64 = (s_3_4.value() as u64);
        // D s_3_6: cast zx s_3_5 -> bv
        let s_3_6: Bits = Bits::new(s_3_5 as u128, 64u16);
        // D s_3_7: not s_3_6
        let s_3_7: Bits = !s_3_6;
        // D s_3_8: cast reint s_3_7 -> u64
        let s_3_8: u64 = (s_3_7.value() as u64);
        // D s_3_9: cast zx s_3_0 -> bv
        let s_3_9: Bits = Bits::new(s_3_0 as u128, 64u16);
        // D s_3_10: cast zx s_3_8 -> bv
        let s_3_10: Bits = Bits::new(s_3_8 as u128, 64u16);
        // D s_3_11: and s_3_9 s_3_10
        let s_3_11: Bits = ((s_3_9) & (s_3_10));
        // D s_3_12: cast reint s_3_11 -> u64
        let s_3_12: u64 = (s_3_11.value() as u64);
        // D s_3_13: call Mk_MPAMIDR_EL1_Type(s_3_12)
        let s_3_13: ProductType5c790c8ef59cc8b2 = Mk_MPAMIDR_EL1_Type(
            state,
            tracer,
            s_3_12,
        );
        // D s_3_14: write-var tmp <= s_3_13
        fn_state.tmp = s_3_13;
        // N s_3_15: jump b2
        return block_2(state, tracer, fn_state);
    }
}
