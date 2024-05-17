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
use Mk_PMSDSFR_EL1_Type::*;
use u__IMPDEF_boolean::*;
use common::*;
pub fn u__get_PMSDSFR_EL1<T: Tracer>(
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
        // C s_0_2: const #"filtering on Data Source [m] is supported" : str
        let s_0_2: &'static str = "filtering on Data Source [m] is supported";
        // S s_0_3: call __IMPDEF_boolean(s_0_2)
        let s_0_3: bool = u__IMPDEF_boolean(state, tracer, s_0_2);
        // S s_0_4: not s_0_3
        let s_0_4: bool = !s_0_3;
        // N s_0_5: branch s_0_4 b3 b1
        if s_0_4 {
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
        // C s_3_1: const #18446744073709551615u : u64
        let s_3_1: u64 = 18446744073709551615;
        // C s_3_2: cast zx s_3_1 -> bv
        let s_3_2: Bits = Bits::new(s_3_1 as u128, 64u16);
        // C s_3_3: not s_3_2
        let s_3_3: Bits = !s_3_2;
        // C s_3_4: cast reint s_3_3 -> u64
        let s_3_4: u64 = (s_3_3.value() as u64);
        // D s_3_5: cast zx s_3_0 -> bv
        let s_3_5: Bits = Bits::new(s_3_0 as u128, 64u16);
        // C s_3_6: cast zx s_3_4 -> bv
        let s_3_6: Bits = Bits::new(s_3_4 as u128, 64u16);
        // D s_3_7: and s_3_5 s_3_6
        let s_3_7: Bits = ((s_3_5) & (s_3_6));
        // D s_3_8: cast reint s_3_7 -> u64
        let s_3_8: u64 = (s_3_7.value() as u64);
        // D s_3_9: call Mk_PMSDSFR_EL1_Type(s_3_8)
        let s_3_9: ProductType5c790c8ef59cc8b2 = Mk_PMSDSFR_EL1_Type(
            state,
            tracer,
            s_3_8,
        );
        // D s_3_10: write-var tmp <= s_3_9
        fn_state.tmp = s_3_9;
        // N s_3_11: jump b2
        return block_2(state, tracer, fn_state);
    }
}
