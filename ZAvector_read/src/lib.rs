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
use CurrentSVL_read::*;
use common::*;
pub fn ZAvector_read<T: Tracer>(
    state: &mut State,
    tracer: &T,
    index: i128,
    width: i128,
) -> Bits {
    #[derive(Default)]
    struct FunctionState {
        gs_22644: bool,
        widthshadow_386: i128,
        index: i128,
        width: i128,
    }
    let fn_state = FunctionState {
        index,
        width,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_0_0: read-var width:i
        let s_0_0: i128 = fn_state.width;
        // D s_0_1: write-var widthshadow#386 <= s_0_0
        fn_state.widthshadow_386 = s_0_0;
        // C s_0_2: const #() : ()
        let s_0_2: () = ();
        // S s_0_3: call CurrentSVL_read(s_0_2)
        let s_0_3: i64 = CurrentSVL_read(state, tracer, s_0_2);
        // S s_0_4: cast zx s_0_3 -> i
        let s_0_4: i128 = (i128::try_from(s_0_3).unwrap());
        // D s_0_5: read-var widthshadow#386:i
        let s_0_5: i128 = fn_state.widthshadow_386;
        // D s_0_6: cmp-eq s_0_5 s_0_4
        let s_0_6: bool = ((s_0_5) == (s_0_4));
        // N s_0_7: assert s_0_6
        let s_0_7: () = assert!(s_0_6);
        // C s_0_8: const #0s : i
        let s_0_8: i128 = 0;
        // D s_0_9: read-var index:i
        let s_0_9: i128 = fn_state.index;
        // D s_0_10: cmp-ge s_0_9 s_0_8
        let s_0_10: bool = ((s_0_9) >= (s_0_8));
        // N s_0_11: branch s_0_10 b3 b1
        if s_0_10 {
            return block_3(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_1_0: const #0u : u8
        let s_1_0: bool = false;
        // D s_1_1: write-var gs#22644 <= s_1_0
        fn_state.gs_22644 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_2_0: read-var gs#22644:u8
        let s_2_0: bool = fn_state.gs_22644;
        // N s_2_1: assert s_2_0
        let s_2_1: () = assert!(s_2_0);
        // C s_2_2: const #23952u : u32
        let s_2_2: u32 = 23952;
        // D s_2_3: read-reg s_2_2:[u2048; 256]
        let s_2_3: [u64; 256usize] = {
            let value = state.read_register::<[u64; 256usize]>(s_2_2 as isize);
            tracer.read_register(s_2_2 as isize, value);
            value
        };
        // D s_2_4: read-var index:i
        let s_2_4: i128 = fn_state.index;
        // D s_2_5: read-element s_2_3[s_2_4]
        let s_2_5: u64 = s_2_3[(s_2_4) as usize];
        // C s_2_6: const #1s : i
        let s_2_6: i128 = 1;
        // D s_2_7: read-var widthshadow#386:i
        let s_2_7: i128 = fn_state.widthshadow_386;
        // D s_2_8: sub s_2_7 s_2_6
        let s_2_8: i128 = ((s_2_7) - (s_2_6));
        // D s_2_9: cast reint s_2_8 -> i64
        let s_2_9: i64 = (s_2_8 as i64);
        // C s_2_10: const #0s : i
        let s_2_10: i128 = 0;
        // D s_2_11: cast zx s_2_5 -> bv
        let s_2_11: Bits = Bits::new(s_2_5 as u128, 2048u16);
        // D s_2_12: cast zx s_2_9 -> i
        let s_2_12: i128 = (i128::try_from(s_2_9).unwrap());
        // C s_2_13: const #1s : i64
        let s_2_13: i64 = 1;
        // C s_2_14: cast zx s_2_13 -> i
        let s_2_14: i128 = (i128::try_from(s_2_13).unwrap());
        // D s_2_15: sub s_2_12 s_2_10
        let s_2_15: i128 = ((s_2_12) - (s_2_10));
        // D s_2_16: add s_2_15 s_2_14
        let s_2_16: i128 = (s_2_15 + s_2_14);
        // D s_2_17: bit-extract s_2_11 s_2_10 s_2_16
        let s_2_17: Bits = (Bits::new(
            ((s_2_11) >> (s_2_10)).value(),
            u16::try_from(s_2_16).unwrap(),
        ));
        // N s_2_18: return s_2_17
        return s_2_17;
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_3_0: const #8s : i
        let s_3_0: i128 = 8;
        // D s_3_1: read-var widthshadow#386:i
        let s_3_1: i128 = fn_state.widthshadow_386;
        // D s_3_2: div s_3_1 s_3_0
        let s_3_2: i128 = ((s_3_1) / (s_3_0));
        // D s_3_3: cast reint s_3_2 -> i64
        let s_3_3: i64 = (s_3_2 as i64);
        // D s_3_4: cast zx s_3_3 -> i
        let s_3_4: i128 = (i128::try_from(s_3_3).unwrap());
        // D s_3_5: read-var index:i
        let s_3_5: i128 = fn_state.index;
        // D s_3_6: cmp-lt s_3_5 s_3_4
        let s_3_6: bool = ((s_3_5) < (s_3_4));
        // D s_3_7: write-var gs#22644 <= s_3_6
        fn_state.gs_22644 = s_3_6;
        // N s_3_8: jump b2
        return block_2(state, tracer, fn_state);
    }
}
