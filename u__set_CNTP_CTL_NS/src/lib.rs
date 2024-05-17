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
use CNTP_CTL_NS_read::*;
use CNTP_CTL_NS_write::*;
use Mk_CNTP_CTL_Type::*;
use common::*;
pub fn u__set_CNTP_CTL_NS<T: Tracer>(
    state: &mut State,
    tracer: &T,
    value_name: ProductType700c18a878c5601b,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        r: ProductType700c18a878c5601b,
        ga_28344: ProductType700c18a878c5601b,
        ga_28338: ProductType700c18a878c5601b,
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
        // C s_0_2: const #() : ()
        let s_0_2: () = ();
        // S s_0_3: call CNTP_CTL_NS_read(s_0_2)
        let s_0_3: ProductType700c18a878c5601b = CNTP_CTL_NS_read(state, tracer, s_0_2);
        // D s_0_4: write-var ga#28338 <= s_0_3
        fn_state.ga_28338 = s_0_3;
        // D s_0_5: read-var ga#28338.0:struct
        let s_0_5: u32 = fn_state.ga_28338._0;
        // D s_0_6: read-var r.0:struct
        let s_0_6: u32 = fn_state.r._0;
        // C s_0_7: const #0s : i
        let s_0_7: i128 = 0;
        // C s_0_8: const #2s : i
        let s_0_8: i128 = 2;
        // D s_0_9: cast zx s_0_6 -> bv
        let s_0_9: Bits = Bits::new(s_0_6 as u128, 32u16);
        // D s_0_10: bit-extract s_0_9 s_0_7 s_0_8
        let s_0_10: Bits = (Bits::new(
            ((s_0_9) >> (s_0_7)).value(),
            u16::try_from(s_0_8).unwrap(),
        ));
        // D s_0_11: cast reint s_0_10 -> u8
        let s_0_11: u8 = (s_0_10.value() as u8);
        // C s_0_12: const #2s : i
        let s_0_12: i128 = 2;
        // C s_0_13: const #0s : i
        let s_0_13: i128 = 0;
        // D s_0_14: cast zx s_0_5 -> bv
        let s_0_14: Bits = Bits::new(s_0_5 as u128, 32u16);
        // D s_0_15: cast zx s_0_11 -> bv
        let s_0_15: Bits = Bits::new(s_0_11 as u128, 2u16);
        // C s_0_16: const #1u : u64
        let s_0_16: u64 = 1;
        // C s_0_17: cast zx s_0_16 -> bv
        let s_0_17: Bits = Bits::new(s_0_16 as u128, 64u16);
        // C s_0_18: lsl s_0_17 s_0_12
        let s_0_18: Bits = s_0_17 << s_0_12;
        // C s_0_19: sub s_0_18 s_0_17
        let s_0_19: Bits = ((s_0_18) - (s_0_17));
        // D s_0_20: and s_0_15 s_0_19
        let s_0_20: Bits = ((s_0_15) & (s_0_19));
        // D s_0_21: lsl s_0_20 s_0_13
        let s_0_21: Bits = s_0_20 << s_0_13;
        // C s_0_22: lsl s_0_19 s_0_13
        let s_0_22: Bits = s_0_19 << s_0_13;
        // C s_0_23: cmpl s_0_22
        let s_0_23: Bits = !s_0_22;
        // D s_0_24: and s_0_14 s_0_23
        let s_0_24: Bits = ((s_0_14) & (s_0_23));
        // D s_0_25: or s_0_24 s_0_21
        let s_0_25: Bits = ((s_0_24) | (s_0_21));
        // D s_0_26: cast reint s_0_25 -> u32
        let s_0_26: u32 = (s_0_25.value() as u32);
        // D s_0_27: call Mk_CNTP_CTL_Type(s_0_26)
        let s_0_27: ProductType700c18a878c5601b = Mk_CNTP_CTL_Type(
            state,
            tracer,
            s_0_26,
        );
        // D s_0_28: call CNTP_CTL_NS_write(s_0_27)
        let s_0_28: () = CNTP_CTL_NS_write(state, tracer, s_0_27);
        // C s_0_29: const #() : ()
        let s_0_29: () = ();
        // S s_0_30: call CNTP_CTL_NS_read(s_0_29)
        let s_0_30: ProductType700c18a878c5601b = CNTP_CTL_NS_read(
            state,
            tracer,
            s_0_29,
        );
        // D s_0_31: write-var ga#28344 <= s_0_30
        fn_state.ga_28344 = s_0_30;
        // D s_0_32: read-var ga#28344.0:struct
        let s_0_32: u32 = fn_state.ga_28344._0;
        // D s_0_33: read-var r.0:struct
        let s_0_33: u32 = fn_state.r._0;
        // C s_0_34: const #3s : i
        let s_0_34: i128 = 3;
        // C s_0_35: const #29s : i
        let s_0_35: i128 = 29;
        // D s_0_36: cast zx s_0_33 -> bv
        let s_0_36: Bits = Bits::new(s_0_33 as u128, 32u16);
        // D s_0_37: bit-extract s_0_36 s_0_34 s_0_35
        let s_0_37: Bits = (Bits::new(
            ((s_0_36) >> (s_0_34)).value(),
            u16::try_from(s_0_35).unwrap(),
        ));
        // D s_0_38: cast reint s_0_37 -> u29
        let s_0_38: u32 = (s_0_37.value() as u32);
        // C s_0_39: const #29s : i
        let s_0_39: i128 = 29;
        // C s_0_40: const #3s : i
        let s_0_40: i128 = 3;
        // D s_0_41: cast zx s_0_32 -> bv
        let s_0_41: Bits = Bits::new(s_0_32 as u128, 32u16);
        // D s_0_42: cast zx s_0_38 -> bv
        let s_0_42: Bits = Bits::new(s_0_38 as u128, 29u16);
        // C s_0_43: const #1u : u64
        let s_0_43: u64 = 1;
        // C s_0_44: cast zx s_0_43 -> bv
        let s_0_44: Bits = Bits::new(s_0_43 as u128, 64u16);
        // C s_0_45: lsl s_0_44 s_0_39
        let s_0_45: Bits = s_0_44 << s_0_39;
        // C s_0_46: sub s_0_45 s_0_44
        let s_0_46: Bits = ((s_0_45) - (s_0_44));
        // D s_0_47: and s_0_42 s_0_46
        let s_0_47: Bits = ((s_0_42) & (s_0_46));
        // D s_0_48: lsl s_0_47 s_0_40
        let s_0_48: Bits = s_0_47 << s_0_40;
        // C s_0_49: lsl s_0_46 s_0_40
        let s_0_49: Bits = s_0_46 << s_0_40;
        // C s_0_50: cmpl s_0_49
        let s_0_50: Bits = !s_0_49;
        // D s_0_51: and s_0_41 s_0_50
        let s_0_51: Bits = ((s_0_41) & (s_0_50));
        // D s_0_52: or s_0_51 s_0_48
        let s_0_52: Bits = ((s_0_51) | (s_0_48));
        // D s_0_53: cast reint s_0_52 -> u32
        let s_0_53: u32 = (s_0_52.value() as u32);
        // D s_0_54: call Mk_CNTP_CTL_Type(s_0_53)
        let s_0_54: ProductType700c18a878c5601b = Mk_CNTP_CTL_Type(
            state,
            tracer,
            s_0_53,
        );
        // D s_0_55: call CNTP_CTL_NS_write(s_0_54)
        let s_0_55: () = CNTP_CTL_NS_write(state, tracer, s_0_54);
        // N s_0_56: return
        return;
    }
}
