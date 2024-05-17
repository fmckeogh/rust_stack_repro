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
use ELUsingAArch32::*;
use EffectiveTBI::*;
use S1TranslationRegime::*;
use common::*;
pub fn AddrTop<T: Tracer>(
    state: &mut State,
    tracer: &T,
    address: u64,
    IsInstr: bool,
    el: u8,
) -> i128 {
    #[derive(Default)]
    struct FunctionState {
        return_value: i128,
        address: u64,
        IsInstr: bool,
        el: u8,
    }
    let fn_state = FunctionState {
        address,
        IsInstr,
        el,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // D s_0_0: read-var el:u8
        let s_0_0: u8 = fn_state.el;
        // C s_0_1: const #2u : u8
        let s_0_1: u8 = 2;
        // D s_0_2: cmp-lt s_0_0 s_0_1
        let s_0_2: bool = ((s_0_0) < (s_0_1));
        // N s_0_3: assert s_0_2
        let s_0_3: () = assert!(s_0_2);
        // D s_0_4: read-var el:u8
        let s_0_4: u8 = fn_state.el;
        // D s_0_5: call S1TranslationRegime(s_0_4)
        let s_0_5: u8 = S1TranslationRegime(state, tracer, s_0_4);
        // D s_0_6: call ELUsingAArch32(s_0_5)
        let s_0_6: bool = ELUsingAArch32(state, tracer, s_0_5);
        // N s_0_7: branch s_0_6 b5 b1
        if s_0_6 {
            return block_5(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // D s_1_0: read-var address:u64
        let s_1_0: u64 = fn_state.address;
        // D s_1_1: read-var IsInstr:u8
        let s_1_1: bool = fn_state.IsInstr;
        // D s_1_2: read-var el:u8
        let s_1_2: u8 = fn_state.el;
        // D s_1_3: call EffectiveTBI(s_1_0, s_1_1, s_1_2)
        let s_1_3: bool = EffectiveTBI(state, tracer, s_1_0, s_1_1, s_1_2);
        // D s_1_4: cast zx s_1_3 -> bv
        let s_1_4: Bits = Bits::new(s_1_3 as u128, 1u16);
        // C s_1_5: const #1u : u8
        let s_1_5: bool = true;
        // C s_1_6: cast zx s_1_5 -> bv
        let s_1_6: Bits = Bits::new(s_1_5 as u128, 1u16);
        // D s_1_7: cmp-eq s_1_4 s_1_6
        let s_1_7: bool = ((s_1_4) == (s_1_6));
        // N s_1_8: branch s_1_7 b4 b2
        if s_1_7 {
            return block_4(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // C s_2_0: const #63s : i
        let s_2_0: i128 = 63;
        // D s_2_1: write-var return_value <= s_2_0
        fn_state.return_value = s_2_0;
        // N s_2_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // D s_3_0: read-var return_value:i
        let s_3_0: i128 = fn_state.return_value;
        // N s_3_1: return s_3_0
        return s_3_0;
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // C s_4_0: const #55s : i
        let s_4_0: i128 = 55;
        // D s_4_1: write-var return_value <= s_4_0
        fn_state.return_value = s_4_0;
        // N s_4_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // C s_5_0: const #31s : i
        let s_5_0: i128 = 31;
        // D s_5_1: write-var return_value <= s_5_0
        fn_state.return_value = s_5_0;
        // N s_5_2: jump b3
        return block_3(state, tracer, fn_state);
    }
}
