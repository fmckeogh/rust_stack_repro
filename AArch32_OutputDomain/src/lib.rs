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
use ConstrainUnpredictableBits::*;
use HaveAArch32EL::*;
use DACR_read::*;
use DACR_NS_read::*;
use common::*;
pub fn AArch32_OutputDomain<T: Tracer>(
    state: &mut State,
    tracer: &T,
    regime: u32,
    domain: u8,
) -> u8 {
    #[derive(Default)]
    struct FunctionState {
        ga_22062: ProductType700c18a878c5601b,
        ga_22064: ProductType700c18a878c5601b,
        Dn: u8,
        index: i64,
        gs_453772: ProductType690b94b58c91cec7,
        regime: u32,
        domain: u8,
    }
    let fn_state = FunctionState {
        regime,
        domain,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_0_0: read-var domain:u8
        let s_0_0: u8 = fn_state.domain;
        // D s_0_1: cast zx s_0_0 -> bv
        let s_0_1: Bits = Bits::new(s_0_0 as u128, 4u16);
        // D s_0_2: cast zx s_0_1 -> i
        let s_0_2: i128 = (s_0_1.value() as i128);
        // D s_0_3: cast reint s_0_2 -> i64
        let s_0_3: i64 = (s_0_2 as i64);
        // C s_0_4: const #2s : i
        let s_0_4: i128 = 2;
        // D s_0_5: cast zx s_0_3 -> i
        let s_0_5: i128 = (i128::try_from(s_0_3).unwrap());
        // D s_0_6: mul s_0_4 s_0_5
        let s_0_6: i128 = ((s_0_4) * (s_0_5));
        // D s_0_7: cast reint s_0_6 -> i64
        let s_0_7: i64 = (s_0_6 as i64);
        // D s_0_8: write-var index <= s_0_7
        fn_state.index = s_0_7;
        // D s_0_9: read-var regime:u32
        let s_0_9: u32 = fn_state.regime;
        // C s_0_10: const #1u : u32
        let s_0_10: u32 = 1;
        // D s_0_11: cmp-eq s_0_9 s_0_10
        let s_0_11: bool = ((s_0_9) == (s_0_10));
        // N s_0_12: branch s_0_11 b8 b1
        if s_0_11 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_1_0: const #424u : u32
        let s_1_0: u32 = 424;
        // D s_1_1: read-reg s_1_0:u8
        let s_1_1: u8 = {
            let value = state.read_register::<u8>(s_1_0 as isize);
            tracer.read_register(s_1_0 as isize, value);
            value
        };
        // D s_1_2: call HaveAArch32EL(s_1_1)
        let s_1_2: bool = HaveAArch32EL(state, tracer, s_1_1);
        // N s_1_3: branch s_1_2 b7 b2
        if s_1_2 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_2_0: const #() : ()
        let s_2_0: () = ();
        // S s_2_1: call DACR_read(s_2_0)
        let s_2_1: ProductType700c18a878c5601b = DACR_read(state, tracer, s_2_0);
        // D s_2_2: write-var ga#22064 <= s_2_1
        fn_state.ga_22064 = s_2_1;
        // D s_2_3: read-var ga#22064.0:struct
        let s_2_3: u32 = fn_state.ga_22064._0;
        // C s_2_4: const #2s : i
        let s_2_4: i128 = 2;
        // D s_2_5: cast zx s_2_3 -> bv
        let s_2_5: Bits = Bits::new(s_2_3 as u128, 32u16);
        // D s_2_6: read-var index:i64
        let s_2_6: i64 = fn_state.index;
        // D s_2_7: cast zx s_2_6 -> i
        let s_2_7: i128 = (i128::try_from(s_2_6).unwrap());
        // D s_2_8: bit-extract s_2_5 s_2_7 s_2_4
        let s_2_8: Bits = (Bits::new(
            ((s_2_5) >> (s_2_7)).value(),
            u16::try_from(s_2_4).unwrap(),
        ));
        // D s_2_9: cast reint s_2_8 -> u8
        let s_2_9: u8 = (s_2_8.value() as u8);
        // D s_2_10: write-var Dn <= s_2_9
        fn_state.Dn = s_2_9;
        // N s_2_11: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_3_0: read-var Dn:u8
        let s_3_0: u8 = fn_state.Dn;
        // D s_3_1: cast zx s_3_0 -> bv
        let s_3_1: Bits = Bits::new(s_3_0 as u128, 2u16);
        // C s_3_2: const #2u : u8
        let s_3_2: u8 = 2;
        // C s_3_3: cast zx s_3_2 -> bv
        let s_3_3: Bits = Bits::new(s_3_2 as u128, 2u16);
        // D s_3_4: cmp-eq s_3_1 s_3_3
        let s_3_4: bool = ((s_3_1) == (s_3_3));
        // N s_3_5: branch s_3_4 b6 b4
        if s_3_4 {
            return block_6(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // N s_4_0: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_5_0: read-var Dn:u8
        let s_5_0: u8 = fn_state.Dn;
        // N s_5_1: return s_5_0
        return s_5_0;
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_6_0: const #2s : i
        let s_6_0: i128 = 2;
        // C s_6_1: const #14u : u32
        let s_6_1: u32 = 14;
        // S s_6_2: call ConstrainUnpredictableBits(s_6_1, s_6_0)
        let s_6_2: ProductType690b94b58c91cec7 = ConstrainUnpredictableBits(
            state,
            tracer,
            s_6_1,
            s_6_0,
        );
        // D s_6_3: write-var gs#453772 <= s_6_2
        fn_state.gs_453772 = s_6_2;
        // D s_6_4: read-var gs#453772.1:struct
        let s_6_4: Bits = fn_state.gs_453772._1;
        // D s_6_5: cast reint s_6_4 -> u8
        let s_6_5: u8 = (s_6_4.value() as u8);
        // D s_6_6: write-var Dn <= s_6_5
        fn_state.Dn = s_6_5;
        // N s_6_7: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_7_0: const #() : ()
        let s_7_0: () = ();
        // S s_7_1: call DACR_NS_read(s_7_0)
        let s_7_1: ProductType700c18a878c5601b = DACR_NS_read(state, tracer, s_7_0);
        // D s_7_2: write-var ga#22062 <= s_7_1
        fn_state.ga_22062 = s_7_1;
        // D s_7_3: read-var ga#22062.0:struct
        let s_7_3: u32 = fn_state.ga_22062._0;
        // C s_7_4: const #2s : i
        let s_7_4: i128 = 2;
        // D s_7_5: cast zx s_7_3 -> bv
        let s_7_5: Bits = Bits::new(s_7_3 as u128, 32u16);
        // D s_7_6: read-var index:i64
        let s_7_6: i64 = fn_state.index;
        // D s_7_7: cast zx s_7_6 -> i
        let s_7_7: i128 = (i128::try_from(s_7_6).unwrap());
        // D s_7_8: bit-extract s_7_5 s_7_7 s_7_4
        let s_7_8: Bits = (Bits::new(
            ((s_7_5) >> (s_7_7)).value(),
            u16::try_from(s_7_4).unwrap(),
        ));
        // D s_7_9: cast reint s_7_8 -> u8
        let s_7_9: u8 = (s_7_8.value() as u8);
        // D s_7_10: write-var Dn <= s_7_9
        fn_state.Dn = s_7_9;
        // N s_7_11: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_8_0: const #11504u : u32
        let s_8_0: u32 = 11504;
        // D s_8_1: read-reg s_8_0:u32
        let s_8_1: u32 = {
            let value = state.read_register::<u32>(s_8_0 as isize);
            tracer.read_register(s_8_0 as isize, value);
            value
        };
        // C s_8_2: const #2s : i
        let s_8_2: i128 = 2;
        // D s_8_3: cast zx s_8_1 -> bv
        let s_8_3: Bits = Bits::new(s_8_1 as u128, 32u16);
        // D s_8_4: read-var index:i64
        let s_8_4: i64 = fn_state.index;
        // D s_8_5: cast zx s_8_4 -> i
        let s_8_5: i128 = (i128::try_from(s_8_4).unwrap());
        // D s_8_6: bit-extract s_8_3 s_8_5 s_8_2
        let s_8_6: Bits = (Bits::new(
            ((s_8_3) >> (s_8_5)).value(),
            u16::try_from(s_8_2).unwrap(),
        ));
        // D s_8_7: cast reint s_8_6 -> u8
        let s_8_7: u8 = (s_8_6.value() as u8);
        // D s_8_8: write-var Dn <= s_8_7
        fn_state.Dn = s_8_7;
        // N s_8_9: jump b3
        return block_3(state, tracer, fn_state);
    }
}
