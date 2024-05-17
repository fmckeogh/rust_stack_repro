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
use MPAM3_EL3_write::*;
use MPAM3_EL3_read::*;
use Mk_MPAM3_EL3_Type::*;
use common::*;
pub fn MPAM1_EL1_write<T: Tracer>(
    state: &mut State,
    tracer: &T,
    value_name: ProductType5c790c8ef59cc8b2,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        r: ProductType5c790c8ef59cc8b2,
        ga_26275: ProductType5c790c8ef59cc8b2,
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
        // C s_0_2: const #424u : u32
        let s_0_2: u32 = 424;
        // D s_0_3: read-reg s_0_2:u8
        let s_0_3: u8 = {
            let value = state.read_register::<u8>(s_0_2 as isize);
            tracer.read_register(s_0_2 as isize, value);
            value
        };
        // C s_0_4: const #2u : u8
        let s_0_4: u8 = 2;
        // D s_0_5: cmp-lt s_0_3 s_0_4
        let s_0_5: bool = ((s_0_3) < (s_0_4));
        // N s_0_6: branch s_0_5 b3 b1
        if s_0_5 {
            return block_3(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var r.0:struct
        let s_1_0: u64 = fn_state.r._0;
        // C s_1_1: const #63s : i
        let s_1_1: i128 = 63;
        // C s_1_2: const #1s : i
        let s_1_2: i128 = 1;
        // D s_1_3: cast zx s_1_0 -> bv
        let s_1_3: Bits = Bits::new(s_1_0 as u128, 64u16);
        // D s_1_4: bit-extract s_1_3 s_1_1 s_1_2
        let s_1_4: Bits = (Bits::new(
            ((s_1_3) >> (s_1_1)).value(),
            u16::try_from(s_1_2).unwrap(),
        ));
        // D s_1_5: cast reint s_1_4 -> u8
        let s_1_5: bool = ((s_1_4.value()) != 0);
        // D s_1_6: call Bit(s_1_5)
        let s_1_6: bool = Bit(state, tracer, s_1_5);
        // C s_1_7: const #90504u : u32
        let s_1_7: u32 = 90504;
        // D s_1_8: read-reg s_1_7:struct
        let s_1_8: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_1_7 as isize);
            tracer.read_register(s_1_7 as isize, value);
            value
        };
        // C s_1_9: const #90504u : u32
        let s_1_9: u32 = 90504;
        // N s_1_10: write-reg s_1_9 <= s_1_8
        let s_1_10: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_1_9 as isize, s_1_8);
            tracer.write_register(s_1_9 as isize, s_1_8);
        };
        // N s_1_11: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var r:struct
        let s_2_0: ProductType5c790c8ef59cc8b2 = fn_state.r;
        // C s_2_1: const #13536u : u32
        let s_2_1: u32 = 13536;
        // N s_2_2: write-reg s_2_1 <= s_2_0
        let s_2_2: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_2_1 as isize, s_2_0);
            tracer.write_register(s_2_1 as isize, s_2_0);
        };
        // N s_2_3: return
        return;
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #() : ()
        let s_3_0: () = ();
        // S s_3_1: call MPAM3_EL3_read(s_3_0)
        let s_3_1: ProductType5c790c8ef59cc8b2 = MPAM3_EL3_read(state, tracer, s_3_0);
        // D s_3_2: write-var ga#26275 <= s_3_1
        fn_state.ga_26275 = s_3_1;
        // D s_3_3: read-var ga#26275.0:struct
        let s_3_3: u64 = fn_state.ga_26275._0;
        // D s_3_4: read-var r.0:struct
        let s_3_4: u64 = fn_state.r._0;
        // C s_3_5: const #63s : i
        let s_3_5: i128 = 63;
        // C s_3_6: const #1s : i
        let s_3_6: i128 = 1;
        // D s_3_7: cast zx s_3_4 -> bv
        let s_3_7: Bits = Bits::new(s_3_4 as u128, 64u16);
        // D s_3_8: bit-extract s_3_7 s_3_5 s_3_6
        let s_3_8: Bits = (Bits::new(
            ((s_3_7) >> (s_3_5)).value(),
            u16::try_from(s_3_6).unwrap(),
        ));
        // D s_3_9: cast reint s_3_8 -> u8
        let s_3_9: bool = ((s_3_8.value()) != 0);
        // C s_3_10: const #1s : i
        let s_3_10: i128 = 1;
        // C s_3_11: const #63s : i
        let s_3_11: i128 = 63;
        // D s_3_12: cast zx s_3_3 -> bv
        let s_3_12: Bits = Bits::new(s_3_3 as u128, 64u16);
        // D s_3_13: cast zx s_3_9 -> bv
        let s_3_13: Bits = Bits::new(s_3_9 as u128, 1u16);
        // C s_3_14: const #1u : u64
        let s_3_14: u64 = 1;
        // C s_3_15: cast zx s_3_14 -> bv
        let s_3_15: Bits = Bits::new(s_3_14 as u128, 64u16);
        // C s_3_16: lsl s_3_15 s_3_10
        let s_3_16: Bits = s_3_15 << s_3_10;
        // C s_3_17: sub s_3_16 s_3_15
        let s_3_17: Bits = ((s_3_16) - (s_3_15));
        // D s_3_18: and s_3_13 s_3_17
        let s_3_18: Bits = ((s_3_13) & (s_3_17));
        // D s_3_19: lsl s_3_18 s_3_11
        let s_3_19: Bits = s_3_18 << s_3_11;
        // C s_3_20: lsl s_3_17 s_3_11
        let s_3_20: Bits = s_3_17 << s_3_11;
        // C s_3_21: cmpl s_3_20
        let s_3_21: Bits = !s_3_20;
        // D s_3_22: and s_3_12 s_3_21
        let s_3_22: Bits = ((s_3_12) & (s_3_21));
        // D s_3_23: or s_3_22 s_3_19
        let s_3_23: Bits = ((s_3_22) | (s_3_19));
        // D s_3_24: cast reint s_3_23 -> u64
        let s_3_24: u64 = (s_3_23.value() as u64);
        // D s_3_25: call Mk_MPAM3_EL3_Type(s_3_24)
        let s_3_25: ProductType5c790c8ef59cc8b2 = Mk_MPAM3_EL3_Type(
            state,
            tracer,
            s_3_24,
        );
        // D s_3_26: call MPAM3_EL3_write(s_3_25)
        let s_3_26: () = MPAM3_EL3_write(state, tracer, s_3_25);
        // N s_3_27: jump b2
        return block_2(state, tracer, fn_state);
    }
}
