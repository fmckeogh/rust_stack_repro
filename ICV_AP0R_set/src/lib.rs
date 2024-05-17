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
pub fn ICV_AP0R_set<T: Tracer>(
    state: &mut State,
    tracer: &T,
    n: i64,
    value_name: u32,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        n: i64,
        value_name: u32,
    }
    let fn_state = FunctionState {
        n,
        value_name,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var value_name:u32
        let s_0_0: u32 = fn_state.value_name;
        // C s_0_1: const #21152u : u32
        let s_0_1: u32 = 21152;
        // D s_0_2: read-reg s_0_1:[u64; 4]
        let s_0_2: [u64; 4usize] = {
            let value = state.read_register::<[u64; 4usize]>(s_0_1 as isize);
            tracer.read_register(s_0_1 as isize, value);
            value
        };
        // D s_0_3: read-var n:i64
        let s_0_3: i64 = fn_state.n;
        // D s_0_4: cast zx s_0_3 -> i
        let s_0_4: i128 = (i128::try_from(s_0_3).unwrap());
        // D s_0_5: read-element s_0_2[s_0_4]
        let s_0_5: u64 = s_0_2[(s_0_4) as usize];
        // C s_0_6: const #0s : i
        let s_0_6: i128 = 0;
        // C s_0_7: const #32s : i
        let s_0_7: i128 = 32;
        // D s_0_8: cast zx s_0_0 -> bv
        let s_0_8: Bits = Bits::new(s_0_0 as u128, 32u16);
        // D s_0_9: bit-extract s_0_8 s_0_6 s_0_7
        let s_0_9: Bits = (Bits::new(
            ((s_0_8) >> (s_0_6)).value(),
            u16::try_from(s_0_7).unwrap(),
        ));
        // D s_0_10: cast reint s_0_9 -> u32
        let s_0_10: u32 = (s_0_9.value() as u32);
        // C s_0_11: const #0s : i
        let s_0_11: i128 = 0;
        // D s_0_12: cast zx s_0_5 -> bv
        let s_0_12: Bits = Bits::new(s_0_5 as u128, 64u16);
        // D s_0_13: cast zx s_0_10 -> bv
        let s_0_13: Bits = Bits::new(s_0_10 as u128, 32u16);
        // C s_0_14: const #31s : i
        let s_0_14: i128 = 31;
        // C s_0_15: const #1u : u64
        let s_0_15: u64 = 1;
        // C s_0_16: cast zx s_0_15 -> bv
        let s_0_16: Bits = Bits::new(s_0_15 as u128, 64u16);
        // C s_0_17: lsl s_0_16 s_0_14
        let s_0_17: Bits = s_0_16 << s_0_14;
        // C s_0_18: sub s_0_17 s_0_16
        let s_0_18: Bits = ((s_0_17) - (s_0_16));
        // D s_0_19: and s_0_13 s_0_18
        let s_0_19: Bits = ((s_0_13) & (s_0_18));
        // D s_0_20: lsl s_0_19 s_0_11
        let s_0_20: Bits = s_0_19 << s_0_11;
        // C s_0_21: lsl s_0_18 s_0_11
        let s_0_21: Bits = s_0_18 << s_0_11;
        // C s_0_22: cmpl s_0_21
        let s_0_22: Bits = !s_0_21;
        // D s_0_23: and s_0_12 s_0_22
        let s_0_23: Bits = ((s_0_12) & (s_0_22));
        // D s_0_24: or s_0_23 s_0_20
        let s_0_24: Bits = ((s_0_23) | (s_0_20));
        // D s_0_25: cast reint s_0_24 -> u64
        let s_0_25: u64 = (s_0_24.value() as u64);
        // C s_0_26: const #21152u : u32
        let s_0_26: u32 = 21152;
        // D s_0_27: read-reg s_0_26:[u64; 4]
        let s_0_27: [u64; 4usize] = {
            let value = state.read_register::<[u64; 4usize]>(s_0_26 as isize);
            tracer.read_register(s_0_26 as isize, value);
            value
        };
        // D s_0_28: read-var n:i64
        let s_0_28: i64 = fn_state.n;
        // D s_0_29: cast zx s_0_28 -> i
        let s_0_29: i128 = (i128::try_from(s_0_28).unwrap());
        // D s_0_30: mutate-element s_0_27[s_0_29] <= s_0_25
        let s_0_30: [u64; 4usize] = {
            let mut local = s_0_27.clone();
            local[(s_0_29) as usize] = s_0_25;
            local
        };
        // D s_0_31: cast cvt s_0_30 -> [u64; 0]
        let s_0_31: alloc::vec::Vec<u64> = alloc::vec::Vec::from(s_0_30);
        // D s_0_32: cast cvt s_0_31 -> [u64; 4]
        let s_0_32: [u64; 4usize] = {
            let mut buf = [Default::default(); 4usize];
            buf.copy_from_slice(&s_0_31);
            buf
        };
        // C s_0_33: const #21152u : u32
        let s_0_33: u32 = 21152;
        // N s_0_34: write-reg s_0_33 <= s_0_32
        let s_0_34: () = {
            state.write_register::<[u64; 4usize]>(s_0_33 as isize, s_0_32);
            tracer.write_register(s_0_33 as isize, s_0_32);
        };
        // C s_0_35: const #20808u : u32
        let s_0_35: u32 = 20808;
        // D s_0_36: read-reg s_0_35:[u32; 4]
        let s_0_36: [u32; 4usize] = {
            let value = state.read_register::<[u32; 4usize]>(s_0_35 as isize);
            tracer.read_register(s_0_35 as isize, value);
            value
        };
        // D s_0_37: read-var n:i64
        let s_0_37: i64 = fn_state.n;
        // D s_0_38: cast zx s_0_37 -> i
        let s_0_38: i128 = (i128::try_from(s_0_37).unwrap());
        // D s_0_39: mutate-element s_0_36[s_0_38] <= s_0_0
        let s_0_39: [u32; 4usize] = {
            let mut local = s_0_36.clone();
            local[(s_0_38) as usize] = s_0_0;
            local
        };
        // D s_0_40: cast cvt s_0_39 -> [u32; 0]
        let s_0_40: alloc::vec::Vec<u32> = alloc::vec::Vec::from(s_0_39);
        // D s_0_41: cast cvt s_0_40 -> [u32; 4]
        let s_0_41: [u32; 4usize] = {
            let mut buf = [Default::default(); 4usize];
            buf.copy_from_slice(&s_0_40);
            buf
        };
        // C s_0_42: const #20808u : u32
        let s_0_42: u32 = 20808;
        // N s_0_43: write-reg s_0_42 <= s_0_41
        let s_0_43: () = {
            state.write_register::<[u32; 4usize]>(s_0_42 as isize, s_0_41);
            tracer.write_register(s_0_42 as isize, s_0_41);
        };
        // N s_0_44: return
        return;
    }
}
