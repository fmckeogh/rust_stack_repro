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
use MPAM3_EL3_read::*;
use common::*;
pub fn MPAM1_EL1_read<T: Tracer>(
    state: &mut State,
    tracer: &T,
    gs_6907: (),
) -> ProductType5c790c8ef59cc8b2 {
    #[derive(Default)]
    struct FunctionState {
        r: ProductType5c790c8ef59cc8b2,
        ga_4678: ProductType5c790c8ef59cc8b2,
        gs_6907: (),
    }
    let fn_state = FunctionState {
        gs_6907,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // C s_0_0: const #13536u : u32
        let s_0_0: u32 = 13536;
        // D s_0_1: read-reg s_0_0:struct
        let s_0_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_0 as isize);
            tracer.read_register(s_0_0 as isize, value);
            value
        };
        // D s_0_2: write-var r <= s_0_1
        fn_state.r = s_0_1;
        // C s_0_3: const #424u : u32
        let s_0_3: u32 = 424;
        // D s_0_4: read-reg s_0_3:u8
        let s_0_4: u8 = {
            let value = state.read_register::<u8>(s_0_3 as isize);
            tracer.read_register(s_0_3 as isize, value);
            value
        };
        // C s_0_5: const #2u : u8
        let s_0_5: u8 = 2;
        // D s_0_6: cmp-lt s_0_4 s_0_5
        let s_0_6: bool = ((s_0_4) < (s_0_5));
        // N s_0_7: branch s_0_6 b3 b1
        if s_0_6 {
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
        // C s_1_0: const #90504u : u32
        let s_1_0: u32 = 90504;
        // D s_1_1: read-reg s_1_0:u64
        let s_1_1: u64 = {
            let value = state.read_register::<u64>(s_1_0 as isize);
            tracer.read_register(s_1_0 as isize, value);
            value
        };
        // C s_1_2: const #63s : i
        let s_1_2: i128 = 63;
        // C s_1_3: const #1s : i
        let s_1_3: i128 = 1;
        // D s_1_4: cast zx s_1_1 -> bv
        let s_1_4: Bits = Bits::new(s_1_1 as u128, 64u16);
        // D s_1_5: bit-extract s_1_4 s_1_2 s_1_3
        let s_1_5: Bits = (Bits::new(
            ((s_1_4) >> (s_1_2)).value(),
            u16::try_from(s_1_3).unwrap(),
        ));
        // D s_1_6: cast reint s_1_5 -> u8
        let s_1_6: bool = ((s_1_5.value()) != 0);
        // D s_1_7: call Bit(s_1_6)
        let s_1_7: bool = Bit(state, tracer, s_1_6);
        // D s_1_8: read-var r:struct
        let s_1_8: ProductType5c790c8ef59cc8b2 = fn_state.r;
        // D s_1_9: write-var r <= s_1_8
        fn_state.r = s_1_8;
        // N s_1_10: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // D s_2_0: read-var r:struct
        let s_2_0: ProductType5c790c8ef59cc8b2 = fn_state.r;
        // N s_2_1: return s_2_0
        return s_2_0;
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5c790c8ef59cc8b2 {
        // C s_3_0: const #() : ()
        let s_3_0: () = ();
        // S s_3_1: call MPAM3_EL3_read(s_3_0)
        let s_3_1: ProductType5c790c8ef59cc8b2 = MPAM3_EL3_read(state, tracer, s_3_0);
        // D s_3_2: write-var ga#4678 <= s_3_1
        fn_state.ga_4678 = s_3_1;
        // D s_3_3: read-var ga#4678.0:struct
        let s_3_3: u64 = fn_state.ga_4678._0;
        // C s_3_4: const #63s : i
        let s_3_4: i128 = 63;
        // C s_3_5: const #1s : i
        let s_3_5: i128 = 1;
        // D s_3_6: cast zx s_3_3 -> bv
        let s_3_6: Bits = Bits::new(s_3_3 as u128, 64u16);
        // D s_3_7: bit-extract s_3_6 s_3_4 s_3_5
        let s_3_7: Bits = (Bits::new(
            ((s_3_6) >> (s_3_4)).value(),
            u16::try_from(s_3_5).unwrap(),
        ));
        // D s_3_8: cast reint s_3_7 -> u8
        let s_3_8: bool = ((s_3_7.value()) != 0);
        // D s_3_9: call Bit(s_3_8)
        let s_3_9: bool = Bit(state, tracer, s_3_8);
        // D s_3_10: read-var r:struct
        let s_3_10: ProductType5c790c8ef59cc8b2 = fn_state.r;
        // D s_3_11: write-var r <= s_3_10
        fn_state.r = s_3_10;
        // N s_3_12: jump b2
        return block_2(state, tracer, fn_state);
    }
}
