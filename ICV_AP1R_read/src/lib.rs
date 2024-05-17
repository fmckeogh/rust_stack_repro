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
use common::*;
pub fn ICV_AP1R_read<T: Tracer>(state: &mut State, tracer: &T, n: i64) -> u32 {
    #[derive(Default)]
    struct FunctionState {
        ga_27148: ProductType5c790c8ef59cc8b2,
        n: i64,
    }
    let fn_state = FunctionState {
        n,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_0_0: const #23016u : u32
        let s_0_0: u32 = 23016;
        // D s_0_1: read-reg s_0_0:[u32; 4]
        let s_0_1: [u32; 4usize] = {
            let value = state.read_register::<[u32; 4usize]>(s_0_0 as isize);
            tracer.read_register(s_0_0 as isize, value);
            value
        };
        // D s_0_2: read-var n:i64
        let s_0_2: i64 = fn_state.n;
        // D s_0_3: cast zx s_0_2 -> i
        let s_0_3: i128 = (i128::try_from(s_0_2).unwrap());
        // D s_0_4: read-element s_0_1[s_0_3]
        let s_0_4: u32 = s_0_1[(s_0_3) as usize];
        // C s_0_5: const #20744u : u32
        let s_0_5: u32 = 20744;
        // D s_0_6: read-reg s_0_5:[struct; 4]
        let s_0_6: [ProductType5c790c8ef59cc8b2; 4usize] = {
            let value = state
                .read_register::<[ProductType5c790c8ef59cc8b2; 4usize]>(s_0_5 as isize);
            tracer.read_register(s_0_5 as isize, value);
            value
        };
        // D s_0_7: read-var n:i64
        let s_0_7: i64 = fn_state.n;
        // D s_0_8: cast zx s_0_7 -> i
        let s_0_8: i128 = (i128::try_from(s_0_7).unwrap());
        // D s_0_9: read-element s_0_6[s_0_8]
        let s_0_9: ProductType5c790c8ef59cc8b2 = s_0_6[(s_0_8) as usize];
        // D s_0_10: write-var ga#27148 <= s_0_9
        fn_state.ga_27148 = s_0_9;
        // D s_0_11: read-var ga#27148.0:struct
        let s_0_11: u64 = fn_state.ga_27148._0;
        // C s_0_12: const #0s : i
        let s_0_12: i128 = 0;
        // C s_0_13: const #32s : i
        let s_0_13: i128 = 32;
        // D s_0_14: cast zx s_0_11 -> bv
        let s_0_14: Bits = Bits::new(s_0_11 as u128, 64u16);
        // D s_0_15: bit-extract s_0_14 s_0_12 s_0_13
        let s_0_15: Bits = (Bits::new(
            ((s_0_14) >> (s_0_12)).value(),
            u16::try_from(s_0_13).unwrap(),
        ));
        // D s_0_16: cast reint s_0_15 -> u32
        let s_0_16: u32 = (s_0_15.value() as u32);
        // C s_0_17: const #0s : i
        let s_0_17: i128 = 0;
        // D s_0_18: cast zx s_0_4 -> bv
        let s_0_18: Bits = Bits::new(s_0_4 as u128, 32u16);
        // D s_0_19: cast zx s_0_16 -> bv
        let s_0_19: Bits = Bits::new(s_0_16 as u128, 32u16);
        // C s_0_20: const #31s : i
        let s_0_20: i128 = 31;
        // C s_0_21: const #1u : u64
        let s_0_21: u64 = 1;
        // C s_0_22: cast zx s_0_21 -> bv
        let s_0_22: Bits = Bits::new(s_0_21 as u128, 64u16);
        // C s_0_23: lsl s_0_22 s_0_20
        let s_0_23: Bits = s_0_22 << s_0_20;
        // C s_0_24: sub s_0_23 s_0_22
        let s_0_24: Bits = ((s_0_23) - (s_0_22));
        // D s_0_25: and s_0_19 s_0_24
        let s_0_25: Bits = ((s_0_19) & (s_0_24));
        // D s_0_26: lsl s_0_25 s_0_17
        let s_0_26: Bits = s_0_25 << s_0_17;
        // C s_0_27: lsl s_0_24 s_0_17
        let s_0_27: Bits = s_0_24 << s_0_17;
        // C s_0_28: cmpl s_0_27
        let s_0_28: Bits = !s_0_27;
        // D s_0_29: and s_0_18 s_0_28
        let s_0_29: Bits = ((s_0_18) & (s_0_28));
        // D s_0_30: or s_0_29 s_0_26
        let s_0_30: Bits = ((s_0_29) | (s_0_26));
        // D s_0_31: cast reint s_0_30 -> u32
        let s_0_31: u32 = (s_0_30.value() as u32);
        // N s_0_32: return s_0_31
        return s_0_31;
    }
}
